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

fn main() {
    println!("Hello, world!");
}
