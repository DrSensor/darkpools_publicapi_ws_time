#[macro_use]
extern crate lazy_static;

mod config;
mod time_data;

pub use bitwyre_ws_core::{debug, error, info, trace, warn};

use bitwyre_ws_core::init_log;
use bitwyre_ws_core::run_periodic_websocket_service;
use bitwyre_ws_core::JsonSerializable;
use bitwyre_ws_core::PeriodicWebsocketConfig;
use bitwyre_ws_core::PeriodicWebsocketState;
use config::is_debugging;
use config::ServiceConfig;
use std::io::Result as IOResult;
use std::sync::Arc;
use time_data::TimeMessage;

fn create_server_time_message() -> String {
    TimeMessage::default().to_json()
}

fn main() -> IOResult<()> {
    init_log(is_debugging(), None);
    lazy_static! {
        static ref CONFIG: ServiceConfig = ServiceConfig::default();
        static ref STATE: PeriodicWebsocketState =
            PeriodicWebsocketState::new(PeriodicWebsocketConfig {
                binding_url: CONFIG.service_baseurl.clone(),
                binding_path: CONFIG.service_path.clone(),
                max_clients: CONFIG.max_clients as usize,
                periodic_interval: CONFIG.blast_interval,
                rapid_request_interval: CONFIG.rapid_request_interval,
                periodic_message_getter: Arc::new(&create_server_time_message),
            });
    }
    run_periodic_websocket_service(Arc::new(&STATE))
}
