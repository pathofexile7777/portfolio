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
    loop {
        // Not necessary
        // let connected_client_count = clients.lock().await.len();
        // if connected_client_count == 0 {
        //     tokio::time::sleep(Duration::from_millis(500)).await;
        //     debug!("No clients connected, skip sending data");
        // }
        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => s, 
            tungstenite::Message::Ping(p) => {
                info!("Ping message received! {:?}", p);
                continue;
            }
            tungstenite::Message::Pong(p) => {
                info!("Pong received: {:?}", p);
                continue;
            }

            _ => {
                error!("Error getting text: {:?}", msg);
                continue;
            }
        };

        info!("msg: {}", msg);
        let parsed: models::DepthStreamWrapper = serde_json::from_str(&msg).expect("Can't parse");

        let pair_key = parsed.stream.split_once('@').unwrap().0;
        pairs_data.insert(pair_key.to_string(), parsed);

        if interval_timer.elapsed().as_millis() < 105 {
            continue;
        }

        let data_copy = pairs_data.clone();
        let triangles = config.triangles.to_vec();
        let cclients = clients.clone();
        tokio::task::spawn(async move {
            for triangle_config in triangles.iter() {
                process_triangle_data(
                    &data_copy, 
                    &triangle_config.pairs[0], 
                    &triangle_config.pairs[1], 
                    &triangle_config.pairs[2], 
                    [
                        &triangle_config.parts[0], 
                        &triangle_config.parts[1], 
                        &triangle_config.parts[2], 
                    ], 
                    cclients.clone(), 
                )
                .await;
            }
        });

        interval_timer = Instant::now();
    }
}

// Here we write a new functio which takes a HashMap of DepthStreamWrapper and taked 3 coin pairs from it and uses that data to call calc_triangle_step
/*
pairs_data = A HashMap of ask and bid data for a particular pair of coins,
start_pair = the first pair of the triangle, eks: ethbtc
mid_pair = the second pair of the triangle, eks: bnbeth
end_pair = the last pair of the triangle, eks: bnbbtc
triangle = array of &str containing symbols that make up the triangle in order !!!
clients = list of connected clients to send the calculated profit data to.
*/

async fn process_triangle_data(
    pairs_data: &HashMap<String, DepthStreamWrapper>, 
    start_pair: &str, 
    mid_pair: &str, 
    end_pair: &str, 
    triangle: [&str; 3], 
    clients: Clients, 
) {
    info!(
        "processing triangle {:?}: {}->{}->{}", 
        triangle, start_pair, mid_pair, end_pair
    );

    // Here we attempt to grap data from the HashMap for the pairs we are interested in and put them in a tuple
    let data = (
        pairs_data.get(start_pair), 
        pairs_data.get(mid_pair), 
        pairs_data.get(end_pair), 
    );
}