use server::*;

#[tokio::main]
async fn main() {
  // let mut game_server = GameServer::create();
  println!("Game server is alive.");

  run_server().await;
}
