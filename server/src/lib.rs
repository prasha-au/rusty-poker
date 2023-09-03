use rand::distributions::{Alphanumeric, DistString};
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use rusty_poker_core::deck::Deck;
use rusty_poker_core::game::{BettingAction, Game, GameState};
use rusty_poker_core::player::Player;
use serde_json::json;
use std::time::Duration;
use tokio::task;

pub struct GameServer {
  id: String,
  // mqtt_client: AsyncClient,
  // mqtt_eventloop: EventLoop,
  game: Game,
  players: Vec<Box<dyn PlayerWithId>>,
}

#[derive(Debug)]
struct PlayerAction {
  player_id: String,
  message: String,
}

impl GameServer {
  pub fn create() -> Self {
    let players: Vec<Box<dyn PlayerWithId>> = vec![Box::new(MqttPlayer::create()), Box::new(MqttPlayer::create())];
    Self {
      id: Alphanumeric.sample_string(&mut rand::thread_rng(), 16),
      game: Game::create(2, 1000),
      players,
    }
  }

  pub async fn run_server(&mut self) {
    let mut mqttoptions = MqttOptions::new("rusty_poker", "broker.hivemq.com", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (mqtt_client, mut mqtt_eventloop) = AsyncClient::new(mqttoptions, 10);

    mqtt_client
      .publish("rusty_poker/game-started", QoS::AtLeastOnce, false, self.id.to_string())
      .await
      .unwrap();

    mqtt_client
      .subscribe(format!("rusty_poker/{}/action/#", self.id), QoS::AtMostOnce)
      .await
      .unwrap();

    let (tx, mut rx) = tokio::sync::mpsc::channel::<PlayerAction>(32);

    // Run the local task set.

    // let mqtt_eventloop = &mut self.mqtt_eventloop;
    let player_ids = self.players.iter().map(|p| p.get_id()).collect::<Vec<_>>();
    let _manager = task::spawn(async move {
      while let Ok(notification) = mqtt_eventloop.poll().await {
        // println!("Received = {:?}", notification);
        if let Event::Incoming(Packet::Publish(msg)) = notification {
          println!("Received message on topic: {}", msg.topic);
          // TODO: This does not have to be reliant on player if we had better topics
          let player_id = player_ids.iter().find(|p| msg.topic.ends_with(*p));
          if let Some(player_id) = player_id {
            println!("Pushing to channel");
            tx.send(PlayerAction {
              player_id: player_id.to_string(),
              message: String::from_utf8(msg.payload.into()).unwrap(),
            })
            .await
            .unwrap();
          }
        }
      }
    });

    println!("Main loop about to run");

    let mut do_stuff = true;
    loop {
      while let Ok(message) = rx.try_recv() {
        println!("Processing queued message {:?}", message);
        let player = self.players.iter_mut().find(|p| p.get_id() == message.player_id);
        if let Some(player) = player {
          player.process_message(&message.message);
        }
      }

      if do_stuff {
        let current_phase = self.game.get_state(None).phase;
        let new_phase = self.game.next();
        if new_phase.is_none() || current_phase != new_phase.unwrap() {
          println!("Phase changed to {:?}", new_phase);
          for player in self.players.iter_mut() {
            player.clear_pending_action();
          }
        }
        self.broadcast_game_state(&mqtt_client).await;
      }

      if let Some(curr_idx) = self.game.get_current_player_index() {
        // println!("waiting for player idx {}", curr_idx);
        let player = &self.players[curr_idx as usize];
        if let Some(action) = player.get_pending_action() {
          self.game.action_current_player(action).unwrap();
          do_stuff = true;
        } else if do_stuff {
          println!("No action for {}", player.get_id());
          mqtt_client
            .publish(
              format!("rusty_poker/{}/request", self.id),
              QoS::AtLeastOnce,
              false,
              player.get_id(),
            )
            .await
            .unwrap();
          do_stuff = false
        }
      }
    }
  }

  async fn broadcast_game_state(&mut self, mqtt_client: &AsyncClient) {
    let game_state = self.game.get_state(None);
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
        format!("rusty_poker/{}/gamestate", self.id),
        QoS::AtLeastOnce,
        false,
        format!("{}", json_state),
      )
      .await
      .unwrap();

    // broadcast player specific state
    if let Some(curr_index) = self.game.get_current_player_index() {
      let game_state = self.game.get_state(Some(curr_index));
      let json_state = json!({
        "wallet": game_state.wallet,
        "hand": deck_to_value(&game_state.hand),
        "value_to_call": game_state.value_to_call,
      });
      mqtt_client
        .publish(
          format!(
            "rusty_poker/{}/player/{}",
            self.id,
            self.players[curr_index as usize].get_id()
          ),
          QoS::AtLeastOnce,
          false,
          format!("{}", json_state),
        )
        .await
        .unwrap();
    }
  }
}

trait PlayerWithId: Player {
  fn get_id(&self) -> String;
  fn get_pending_action(&self) -> Option<BettingAction>;
  fn clear_pending_action(&mut self);
  fn process_message(&mut self, request: &str);
}

pub struct MqttPlayer {
  pub id: String,
  last_action: Option<BettingAction>,
}

impl MqttPlayer {
  pub fn create() -> Self {
    Self {
      id: Alphanumeric.sample_string(&mut rand::thread_rng(), 5).to_lowercase(),
      last_action: None,
    }
  }

  pub fn process_message(&mut self, message: String) {
    println!("Received message: {}", message);
  }
}

impl PlayerWithId for MqttPlayer {
  fn get_id(&self) -> String {
    self.id.clone()
  }

  fn get_pending_action(&self) -> Option<BettingAction> {
    self.last_action
  }

  fn clear_pending_action(&mut self) {
    self.last_action = None;
  }

  fn process_message(&mut self, request: &str) {
    let split_request: Vec<_> = request.split(' ').collect();
    self.last_action = match split_request[0] {
      "raise" => Some(BettingAction::Raise(split_request[1].parse::<u32>().unwrap())),
      "allin" => Some(BettingAction::AllIn),
      "call" => Some(BettingAction::Call),
      "fold" => Some(BettingAction::Fold),
      &_ => None,
    };
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
