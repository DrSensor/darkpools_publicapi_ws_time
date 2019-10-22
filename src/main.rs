mod config;
mod time_data;

pub use bitwyre_ws_core::{debug, error, info, trace, warn};

use bitwyre_ws_core::init_log;
use bitwyre_ws_core::JsonSerializable;
use config::is_debugging;
use time_data::TimeMessage;

fn create_server_time_message() -> String {
    TimeMessage::default().to_json()
}

fn main() {
    init_log(is_debugging(), None);
    info!("Current server time: {}", create_server_time_message());
}
