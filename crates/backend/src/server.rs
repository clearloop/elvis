use crate::{client, err::Error, logger::Logger, manifest::Crate};
use futures::join;
use std::sync::{mpsc::channel, Arc, Mutex};
use warp::{ws::Ws, Filter};

/// Serve the backend
#[tokio::main]
pub async fn run(mani: Crate, port: u16) -> Result<(), Error> {
    let (tx, rx) = channel();
    let rx = Arc::new(Mutex::new(rx));
    let rx = warp::any().map(move || rx.clone());

    let index = warp::filters::fs::dir(mani.wasm().clone());
    let updater = warp::path("updater")
        .and(warp::ws())
        .and(rx)
        .map(|ws: Ws, rx| ws.on_upgrade(move |socket| client::connect(socket, rx)));

    // dev http server
    logger!(Logger::ServerStart, port);
    let server = warp::serve(index.or(updater)).run(([0, 0, 0, 0], port));

    // file watcher
    let watcher = tokio::task::spawn_blocking(move || mani.watch(tx));
    if let Err(e) = join!(watcher, server).0 {
        return Err(Error::Custom(e.to_string()));
    }

    Ok(())
}
