use rand::distributions::{Alphanumeric, DistString};
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use rusty_poker_core::deck::Deck;
use rusty_poker_core::game::{BettingAction, Game, GameState};
use rusty_poker_core::player::{BasicPlayer, Player};
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::task;

pub async fn run_server() {
  let total_players = 4;
  let game_id = "test"; //Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
  let mut game = Game::create(total_players, 1000);

  let mut mqttoptions = MqttOptions::new("rusty_poker", "broker.hivemq.com", 1883);
  mqttoptions.set_keep_alive(Duration::from_secs(5));
  let (mqtt_client, mut mqtt_eventloop) = AsyncClient::new(mqttoptions, 10);

  mqtt_client
    .publish("rusty_poker/game-started", QoS::AtLeastOnce, false, game_id.to_string())
    .await
    .unwrap();

  mqtt_client
    .subscribe(format!("rusty_poker/{}/action/#", game_id), QoS::AtMostOnce)
    .await
    .unwrap();

  let mqtt_players = vec![MqttPlayer::create(), MqttPlayer::create()];
  let mqtt_player_ids = mqtt_players.iter().map(|p| p.get_id()).collect::<Vec<_>>();

  let basic_players_iter =
    ((mqtt_player_ids.len() as u8)..total_players).map(|id| Box::new(BasicPlayer { id }) as Box<dyn ExtendedPlayer>);

  let players_arc: Vec<RwLock<Box<dyn ExtendedPlayer>>> = mqtt_players
    .into_iter()
    .map(|v| Box::new(v) as Box<dyn ExtendedPlayer>)
    .chain(basic_players_iter)
    .map(RwLock::new)
    .collect::<Vec<_>>();
  let players_arc = Arc::new(players_arc);

  let local = task::LocalSet::new();

  let mut mqtt_player_ids_map: HashMap<String, usize> = HashMap::new();
  for (idx, player_id) in mqtt_player_ids.iter().enumerate() {
    mqtt_player_ids_map.insert(player_id.to_string(), idx);
  }

  let player_arcs_clone = players_arc.clone();
  local.spawn_local(async move {
    while let Ok(notification) = mqtt_eventloop.poll().await {
      // println!("Received = {:?}", notification);
      if let Event::Incoming(Packet::Publish(msg)) = notification {
        println!("Received message on topic: {}", msg.topic);
        if !msg.topic.contains("/action/") {
          continue;
        }
        println!("It's an action message");
        let player_idx = mqtt_player_ids_map.get(msg.topic.split('/').nth(3).unwrap_or(""));
        if player_idx.is_none() {
          continue;
        }
        println!("It is for player {}", player_idx.unwrap());

        let mut player = player_arcs_clone[*player_idx.unwrap()].write().unwrap();
        println!("Processing message for mqtt player: {}", player.get_id());
        player.process_message(&String::from_utf8(msg.payload.into()).unwrap());
      }
    }
  });

  let player_arcs_clone = players_arc.clone();
  local.spawn_local(async move {
    let player_ids = players_arc
      .iter()
      .map(|p| p.read().unwrap().get_id())
      .collect::<Vec<_>>();
    loop {
      let current_phase = game.get_state(None).phase;
      let new_phase = game.next();
      if new_phase.is_none() || current_phase != new_phase.unwrap() {
        for player in player_arcs_clone.iter() {
          player.write().unwrap().clear_pending_action();
        }
      }

      broadcast_game_state(&game, &player_ids, &mqtt_client, game_id).await;

      if let Some(curr_idx) = game.get_current_player_index() {
        // println!("waiting for player idx {}", curr_idx);
        let player_arc = &player_arcs_clone[curr_idx as usize];
        let player = player_arc.read().unwrap();

        println!("Checking action for {}", player.get_id().clone());
        if let Some(action_rx) = player.get_action_rx() {
          println!("    we are waiting for an mqtt message...");
          drop(player);
          let action = action_rx.clone().wait_for(|v| v.is_some()).await.unwrap().unwrap();
          game.action_current_player(action).unwrap();
        } else {
          let action = player.request_action(game.get_state(Some(curr_idx)));
          drop(player);
          println!("    we will action automatically...");
          game.action_current_player(action).unwrap();
        }
        let mut player = player_arc.write().unwrap();
        player.clear_pending_action();
      }
    }
  });

  local.await;
}

async fn broadcast_game_state(game: &Game, player_ids: &[String], mqtt_client: &AsyncClient, game_id: &str) {
  let game_state = game.get_state(None);
  let json_state = json!({
    "total_pot": game_state.total_pot,
    "players": game_state.players.iter().map(|p| json!({
      "wallet": p.wallet,
      "is_folded": p.is_folded,
      "money_on_table": p.money_on_table,
    })).collect::<Vec<_>>(),
    "dealer_index": game_state.dealer_index,
    "current_player_index": game_state.current_player_index,
    "table": deck_to_value(&game_state.table),
  });
  mqtt_client
    .publish(
      format!("rusty_poker/{}/gamestate", game_id),
      QoS::AtLeastOnce,
      false,
      format!("{}", json_state),
    )
    .await
    .unwrap();

  // broadcast player specific state
  if let Some(curr_index) = game.get_current_player_index() {
    let game_state = game.get_state(Some(curr_index));
    let json_state = json!({
      "wallet": game_state.wallet,
      "hand": deck_to_value(&game_state.hand),
      "value_to_call": game_state.value_to_call,
    });
    mqtt_client
      .publish(
        format!("rusty_poker/{}/player/{}", game_id, player_ids[curr_index as usize],),
        QoS::AtLeastOnce,
        false,
        format!("{}", json_state),
      )
      .await
      .unwrap();
  }
}

trait ExtendedPlayer: Player {
  fn get_id(&self) -> String;
  fn get_action_rx(&self) -> Option<tokio::sync::watch::Receiver<Option<BettingAction>>> {
    None
  }
  fn clear_pending_action(&mut self) {}
  fn process_message(&mut self, _request: &str) {}
}

pub struct MqttPlayer {
  pub id: String,
  action_tx: tokio::sync::watch::Sender<Option<BettingAction>>,
  pub action_rx: tokio::sync::watch::Receiver<Option<BettingAction>>,
}

impl MqttPlayer {
  pub fn create() -> Self {
    let (tx, rx) = tokio::sync::watch::channel(None);
    Self {
      id: Alphanumeric.sample_string(&mut rand::thread_rng(), 5).to_lowercase(),
      action_tx: tx,
      action_rx: rx,
    }
  }
}

impl ExtendedPlayer for MqttPlayer {
  fn get_id(&self) -> String {
    self.id.clone()
  }

  fn get_action_rx(&self) -> Option<tokio::sync::watch::Receiver<Option<BettingAction>>> {
    Some(self.action_rx.clone())
  }

  fn clear_pending_action(&mut self) {
    self.action_tx.send_replace(None);
  }

  fn process_message(&mut self, request: &str) {
    let split_request: Vec<_> = request.split(' ').collect();
    self
      .action_tx
      .send(match split_request[0] {
        "raise" => Some(BettingAction::Raise(split_request[1].parse::<u32>().unwrap())),
        "allin" => Some(BettingAction::AllIn),
        "call" => Some(BettingAction::Call),
        "fold" => Some(BettingAction::Fold),
        &_ => None,
      })
      .unwrap();
  }
}

impl Player for MqttPlayer {
  fn request_action(&self, _info: GameState) -> BettingAction {
    BettingAction::Fold
  }
}

fn deck_to_value(deck: &Deck) -> serde_json::Value {
  json!(deck.get_cards().iter().map(|c| c.to_string()).collect::<Vec<_>>())
}

impl ExtendedPlayer for BasicPlayer {
  fn get_id(&self) -> String {
    format!("ai_{}", self.id.clone())
  }
}
