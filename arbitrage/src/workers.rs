use crate::{
    config::AppConfig, 
    models::{self, DepthStreamWrapper}, 
    Clients, 
};
use log::{debug, error, info};
use std::{collections::HashMap, net::TcpStream};
use tokio::time::{Duration, Instant};

use tungstenite::{protocol::WebSocket, stream::MaybeTlsStream};

/// Here we have an infinite loop that "gather data"
/// for our crypto triangle arbitrage and send it to
/// connected clients every 0.5 sec.
pub async fn main_worker(
    clients: Clients, 
    config: AppConfig, 
    mut socket: WebSocket<MaybeTlsStream<TcpStream>>, 
) {
    let mut pairs_data: HashMap<String, DepthStreamWrapper> = HashMap::new();
    let mut interval_timer = Instant::now();
}