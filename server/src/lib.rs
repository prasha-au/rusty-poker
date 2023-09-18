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

enum PlayerType {
  // TODO: Can we somehow get away with the rwlock wrap?
  BasicPlayer(RwLock<BasicPlayer>),
  MqttPlayer(RwLock<MqttPlayer>),
}

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
    ((mqtt_players.len() as u8)..total_players).map(|id| PlayerType::BasicPlayer(RwLock::new(BasicPlayer { id })));

  let players_arc: Vec<PlayerType> = mqtt_players
    .into_iter()
    .map(|v| PlayerType::MqttPlayer(RwLock::new(v)))
    .chain(basic_players_iter)
    .collect::<Vec<_>>();
  let players_arc = Arc::new(players_arc);

  let local = task::LocalSet::new();

  // TODO: This can move into the spawn with the enum changes
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

        if let PlayerType::MqttPlayer(player_lock) = &player_arcs_clone[*player_idx.unwrap()] {
          let mut player = player_lock.write().unwrap();
          println!("Processing message for mqtt player: {}", player.get_id());
          player.process_message(&String::from_utf8(msg.payload.into()).unwrap());
        }
      }
    }
  });

  let player_arcs_clone = players_arc.clone();
  local.spawn_local(async move {
    loop {
      let current_phase = game.get_state(None).phase;
      let new_phase = game.next();
      if new_phase.is_none() || current_phase != new_phase.unwrap() {
        for player in player_arcs_clone.iter() {
          if let PlayerType::MqttPlayer(player) = player {
            player.write().unwrap().clear_pending_action();
          }
        }
      }

      broadcast_game_state(&game, &mqtt_client, game_id).await;

      let curr_idx = game.get_current_player_index();
      if curr_idx.is_none() {
        continue;
      }
      let curr_idx = curr_idx.unwrap();

      // println!("waiting for player idx {}", curr_idx);
      match &player_arcs_clone[curr_idx as usize] {
        PlayerType::MqttPlayer(player_lock) => {
          let player = player_lock.read().unwrap();
          let player_id = player.get_id();
          broadcast_player_state(&game.get_state(Some(curr_idx)), &player_id, &mqtt_client, game_id).await;

          println!("Checking action for {}", player_id);

          let action_rx = player.get_action_rx();
          println!("    we are waiting for an mqtt message...");
          drop(player);
          let action = action_rx.clone().wait_for(|v| v.is_some()).await.unwrap().unwrap();
          game.action_current_player(action).unwrap();

          let mut player = player_lock.write().unwrap();
          player.clear_pending_action();
        }
        PlayerType::BasicPlayer(player_lock) => {
          let player = player_lock.write().unwrap();
          let game_state = game.get_state(Some(curr_idx));
          game.action_current_player(player.request_action(game_state)).unwrap();
        }
      }
    }
  });

  local.await;
}

async fn broadcast_game_state(game: &Game, mqtt_client: &AsyncClient, game_id: &str) {
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
}

async fn broadcast_player_state(game_state: &GameState, player_id: &str, mqtt_client: &AsyncClient, game_id: &str) {
  let json_state = json!({
    "wallet": game_state.wallet,
    "hand": deck_to_value(&game_state.hand),
    "value_to_call": game_state.value_to_call,
  });
  mqtt_client
    .publish(
      format!("rusty_poker/{}/player/{}", game_id, player_id),
      QoS::AtLeastOnce,
      false,
      format!("{}", json_state),
    )
    .await
    .unwrap();
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

  fn get_action_rx(&self) -> tokio::sync::watch::Receiver<Option<BettingAction>> {
    self.action_rx.clone()
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

  fn get_id(&self) -> String {
    self.id.clone()
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
