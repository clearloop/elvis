#![macro_use]

macro_rules! logger {
    ($l: expr, $p: expr) => {
        match $l {
            Logger::ServerStart => trace!("Development server start at 0.0.0.0:{:?}", $p),
            Logger::ServerStartFailed => error!("Development server start failed: {:?}", $p),
            Logger::WebsocketSubscribeFailed => error!("Subscribe update failed: {:?}", $p),
        }
    };
}

/// Backend loggers
pub enum Logger {
    ServerStart,
    ServerStartFailed,
    WebsocketSubscribeFailed,
}
