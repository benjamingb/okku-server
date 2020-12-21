#[macro_use]
extern crate log;

use std::env;

mod database;
mod error;
mod hub;
mod middleware;
mod model;
mod proto;
mod server;
mod service;

#[tokio::main]
async fn main() {
    dotenv::dotenv()
        .ok()
        .expect("Unable to find .env file. Create one based on the .env.sample");

    env_logger::init();

    let port = env::var("PORT")
        .expect("Missing PORT environment variable")
        .parse::<u16>()
        .expect("Invalid PORT value, expected u16");

    database::ping().await;

    let server = server::Server::new(port);

    info!(
        "{}",
        format!("Server listening on: ws://127.0.0.1:{}", port)
    );

    server.run().await;
}
