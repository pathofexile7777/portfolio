use log::{debug, info};
use tungstenite::client;
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
    let f = std::fs::File::open("config.yaml").expect("Could not open file.");
    let app_config: config::AppConfig = serde_yaml::from_reader(f).expect("Could not read values.");

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    // Configure the websocket route, pass a clone of the clients hashmap to use in the ws_handler function.
    // Cloning creates a reference to the original clients, then configure cors on all routes.
    info!("Configuring websocket route");
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and_then(handlers::ws_handler);

    let routes = ws_route.with(warp::cors().allow_any_origin());

    info!("Connecting to binance stream...");
    let binance_url = get_binance_streams_url(
        &app_config.depth_streams, 
        app_config.update_interval, 
        app_config.results_limit, 
    );

    info!("Subscribing to binance: {}", binance_url);
    let (socket, response) = tungstenite::connect(binance_url).expect("Can't connect.");
    info!("Connected to binance stream.");
    debug!("HTTP status code: {}", response.status());
    debug!("Response headers:");
    for (ref header, ref header_value) in response.headers() {
        debug!("- {}: {:?}", header, header_value);
    }

    // Using spawn we create a new task in a separate green thread, we pass the list of connected clients to the main_worker function
    // This function contains the data update loop that sends data to all the connected clients in the list
    info!("Starting update loop");
    tokio::task::spawn(async move {
        workers::main_worker(clients.clone(), app_config, socket).await;
    });
    info!("Starting server");
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}

// ????
fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients, ), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}