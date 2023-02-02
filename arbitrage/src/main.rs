use log::{debug, info};
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use url::Url;
use warp::{ws::Message, Filter, Rejection};

mod config;
mod handlers;
mod models;
mod workers;
mod ws;

static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";

// The client struct represents information about a connected client. The client_id field is a randomly generated
// uuid string. The sender field allows us to send data to the client.

#[derive(Debug, Clone)]
pub struct Client {
    pub client_id: String, 
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>, 
}

// The alias type Clients represents a thread safe, reference tracked hashmap that keeps track of the connected clients
type Clients = Arc<Mutex<HashMap<String, Client>>>;
type Result<T> = std::result::Result<T, Rejection>;

fn get_binance_streams_url(
    depth_streams: &Vec<String>, 
    update_interval: u32, 
    results_limit: u32, 
) -> Url {
    let mut depth_streams_parts: Vec<String> = vec![];
    for stream in depth_streams {
        depth_streams_parts.push(format!(
            "{}@depth{}@{}ms", 
            stream, results_limit, update_interval
        ));
    }

    let depth_streams_joined = depth_streams_parts.join("/");
    let binance_url = format!("{}/stream?streams={}", BINANCE_WS_API, depth_streams_joined);

    Url::parse(&binance_url).unwrap()
}

#[tokio::main]
async fn main() {
    log4rs::init_file("log_config.yaml", Default::default()).unwrap();
}
