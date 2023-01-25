use crate::{ws, Clients, Result};
use log::debug;
use warp::Reply;

/// code to upgrade the connection to a WebSocket connection is initiated
/// and a function is called to do further handling of the WebSocket Connection ws::client_connection

pub async fn ws_handler(ws: warp::ws::Ws, clients: Clients) -> Result<impl Reply> {
    debug!("ws_handler");
    Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, clients)))
}