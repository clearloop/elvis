use crate::logger::Logger;
use futures::{sink::SinkExt, StreamExt};
use std::sync::{mpsc::Receiver, Arc, Mutex};
use warp::ws::{Message, WebSocket};

/// Handle the updater
pub async fn connect(ws: WebSocket, rx: Arc<Mutex<Receiver<bool>>>) {
    let (mut tx, _) = ws.split();

    tokio::task::spawn_blocking(move || loop {
        if rx.lock().unwrap().recv().is_ok() {
            if let Err(e) =
                tokio::runtime::Handle::current().block_on(tx.send(Message::text("update")))
            {
                logger!(Logger::WebsocketSubscribeFailed, e);
            }
        }
    });
}
