use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use futures::{FutureExt, StreamExt};
use tokio::sync::{mpsc, RwLock};
use warp::ws::{Message, WebSocket};

/// Our global unique client id counter.
static NEXT_CLIENT_ID: AtomicUsize = AtomicUsize::new(1);

/// Our state of currently connected clients.
///
/// - Key is their id
/// - Value is a sender of `warp::ws::Message`
pub type Clients = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;

pub async fn client_connected(ws: WebSocket, clients: Clients) {
    // Use a counter to assign a new unique ID for this client.
    let my_id = NEXT_CLIENT_ID.fetch_add(1, Ordering::Relaxed);

    eprintln!("new chat client: {}", my_id);

    // Split the socket into a sender and receive of messages.
    let (client_ws_tx, mut client_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
    let (tx, rx) = mpsc::unbounded_channel();
    tokio::task::spawn(rx.forward(client_ws_tx).map(|result| {
        if let Err(e) = result {
            eprintln!("websocket send error: {}", e);
        }
    }));

    // Save the sender in our list of connected clients.
    clients.write().await.insert(my_id, tx);

    // Return a `Future` that is basically a state machine managing
    // this specific client's connection.

    // Make an extra clone to give to our disconnection handler...
    let clients2 = clients.clone();

    // Every time the client sends a message, broadcast it to
    // all other clients...
    while let Some(result) = client_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(uid={}): {}", my_id, e);
                break;
            }
        };
        client_message(my_id, msg, &clients).await;
    }

    // client_ws_rx stream will keep processing as long as the client stays
    // connected. Once they disconnect, then...
    client_disconnected(my_id, &clients2).await;
}

async fn client_message(my_id: usize, msg: Message, clients: &Clients) {
    // Skip any non-Text messages...
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    let new_msg = format!("<Client#{}>: {}", my_id, msg);

    // New message from this client, send it to everyone else (except same uid)...
    for (&uid, tx) in clients.read().await.iter() {
        if my_id != uid {
            if let Err(_disconnected) = tx.send(Ok(Message::text(new_msg.clone()))) {
                // The tx is disconnected, our `client_disconnected` code
                // should be happening in another task, nothing more to
                // do here.
            }
        }
    }
}

async fn client_disconnected(my_id: usize, clients: &Clients) {
    eprintln!("good bye client: {}", my_id);

    // Stream closed up, so remove from the client list
    clients.write().await.remove(&my_id);
}
