use rand::distributions::{Alphanumeric, DistString};
use rumqttc::{AsyncClient, Event, EventLoop, MqttOptions, Packet, QoS};
use rusty_poker_core::deck::Deck;
use rusty_poker_core::game::{BettingAction, Game, GameState};
use rusty_poker_core::player::Player;
use serde_json::json;
use std::time::Duration;

pub struct GameServer {
  mqtt_client: AsyncClient,
  mqtt_eventloop: EventLoop,
  game: Game,
  players: Vec<Box<dyn PlayerWithId>>,
}

trait PlayerWithId: Player {
  fn get_id(&self) -> String;
  fn get_action_from_message(&self, request: &str) -> BettingAction;
}

impl GameServer {
  pub fn create() -> Self {
    let mut mqttoptions = MqttOptions::new("rusty_poker", "broker.hivemq.com", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (mqtt_client, mqtt_eventloop) = AsyncClient::new(mqttoptions, 10);

    let players: Vec<Box<dyn PlayerWithId>> = vec![Box::new(MqttPlayer::create()), Box::new(MqttPlayer::create())];

    Self {
      mqtt_client,
      mqtt_eventloop,
      game: Game::create(2, 1000),
      players,
    }
  }

  pub async fn run_server(&mut self) {
    self
      .mqtt_client
      .publish("rusty_poker/hello", QoS::AtLeastOnce, false, "game-started")
      .await
      .unwrap();

    // tokio::spawn(async move {
    //   let mut player1 = MqttPlayer::from_connection("player1".to_string(), c1, self.mqtt_eventloop);
    //   player1.run().await.unwrap();
    // });

    self
      .mqtt_client
      .subscribe("rusty_poker/#", QoS::AtMostOnce)
      .await
      .unwrap();

    loop {
      if let Some(curr_index) = self.game.get_current_player_index() {
        let player = self.players[curr_index as usize].as_ref();
        let player_id = player.get_id();
        println!("Waiting for player id {}", &player_id);
        let action = self
          .wait_for_message(format!("rusty_poker/{}", &player_id).as_str())
          .await;
        println!("Got an action for player id {}: {}", &player_id, action);
        self
          .game
          .action_current_player(self.players[curr_index as usize].get_action_from_message(action.as_str()))
          .unwrap();
      }
      self.game.next();
      self.broadcast_game_state().await;
    }
  }

  async fn broadcast_game_state(&mut self) {
    let game_state = self.game.get_state(None);
    let json_state = json!({
      "total_pot": game_state.total_pot,
      "money_on_table": game_state.players.iter().map(|p| p.money_on_table).collect::<Vec<_>>(),
      "dealer_index": game_state.dealer_index,
      "current_player_index": game_state.current_player_index,
      "table": deck_to_value(&game_state.table),
    });
    self
      .mqtt_client
      .publish(
        "rusty_poker/gamestate",
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
      self
        .mqtt_client
        .publish(
          format!("rusty_poker/gamestate/{}", self.players[curr_index as usize].get_id()),
          QoS::AtLeastOnce,
          false,
          format!("{}", json_state),
        )
        .await
        .unwrap();
    }
  }

  async fn wait_for_message(&mut self, topic: &str) -> String {
    while let Ok(notification) = self.mqtt_eventloop.poll().await {
      println!("Received = {:?}", notification);
      if let Event::Incoming(Packet::Publish(msg)) = notification {
        println!("Received message on topic: {}", msg.topic);
        if msg.topic == topic {
          return String::from_utf8(msg.payload.into()).unwrap();
        }
      }
    }
    String::from("woah hello")
  }
}

pub struct MqttPlayer {
  pub id: String,
}

impl MqttPlayer {
  pub fn create() -> Self {
    let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    println!("{}", string);
    Self { id: string }
  }

  pub fn process_message(&mut self, message: String) {
    println!("Received message: {}", message);
  }
}

impl PlayerWithId for MqttPlayer {
  fn get_id(&self) -> String {
    self.id.clone()
  }

  fn get_action_from_message(&self, request: &str) -> BettingAction {
    let split_request: Vec<_> = request.split(" ").collect();
    match split_request[0] {
      "raise" => BettingAction::Raise(split_request[1].parse::<u32>().unwrap()),
      "allin" => BettingAction::AllIn,
      "call" => BettingAction::Call,
      "fold" => BettingAction::Fold,
      &_ => BettingAction::Fold,
    }
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
