use rand::distributions::{Alphanumeric, DistString};
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use rusty_poker_core::deck::Deck;
use rusty_poker_core::game::{BettingAction, Game, GameState};
use rusty_poker_core::player::Player;
use serde_json::json;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::task;

pub async fn run_server() {
  let players: Vec<MqttPlayer> = vec![MqttPlayer::create(), MqttPlayer::create()];
  let game_id = "test"; //Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
  let mut game = Game::create(2, 1000);

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

  let player_ids_arc = Arc::new(players.iter().map(|p| p.get_id()).collect::<Vec<_>>());
  let players_arc = Arc::new(RwLock::new(players));

  let local = task::LocalSet::new();


  let players_for_spawn = players_arc.clone();
  let player_ids = player_ids_arc.clone();
  local.spawn_local(async move {
    while let Ok(notification) = mqtt_eventloop.poll().await {
      // println!("Received = {:?}", notification);
      if let Event::Incoming(Packet::Publish(msg)) = notification {
        println!("Received message on topic: {}", msg.topic);
        // TODO: This does not have to be reliant on player if we had better topics
        let player_idx = player_ids.iter().position(|p| msg.topic.ends_with(p));
        if let Some(player_idx) = player_idx {
          let message = String::from_utf8(msg.payload.into()).unwrap();
          println!("Calling action for player idx {}: {}", player_idx, message);
          let mut players = players_for_spawn.write().unwrap();
          players[player_idx].process_message(&message);
        }
      }
    }
  });

  let player_ids = player_ids_arc.clone();
  local.spawn_local(async move {
    loop {
      let current_phase = game.get_state(None).phase;
      let new_phase = game.next();
      if new_phase.is_none() || current_phase != new_phase.unwrap() {
        let mut players = players_arc.write().unwrap();
        println!("Phase changed to {:?}", new_phase);
        for player in players.iter_mut() {
          player.clear_pending_action();
        }
        drop(players);
      }

      broadcast_game_state(&game, &player_ids, &mqtt_client, game_id).await;

      if let Some(curr_idx) = game.get_current_player_index() {
        let players = players_arc.read().unwrap();
        // println!("waiting for player idx {}", curr_idx);
        let player = &players[curr_idx as usize];

        println!("Checking action for {}", player.get_id().clone());
        let mut action_rec = player.action_rx.clone();
        drop(players);
        let action = action_rec.wait_for(|v| v.is_some()).await.unwrap().unwrap();
        game.action_current_player(action).unwrap();

        let mut players = players_arc.write().unwrap();
        let player = &mut players[curr_idx as usize];
        player.clear_pending_action();
      }
    }
  });

  local.await;
}



async fn broadcast_game_state(game: &Game, player_ids: &Vec<String>, mqtt_client: &AsyncClient, game_id: &str) {
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
        format!(
          "rusty_poker/{}/player/{}",
          game_id,
          player_ids[curr_index as usize],
        ),
        QoS::AtLeastOnce,
        false,
        format!("{}", json_state),
      )
      .await
      .unwrap();
  }
}

trait PlayerWithId: Player {
  fn get_id(&self) -> String;
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

  fn clear_pending_action(&mut self) {
    self.action_tx.send_replace(None);
  }
}

impl PlayerWithId for MqttPlayer {
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
