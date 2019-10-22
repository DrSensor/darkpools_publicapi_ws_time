use bitwyre_ws_core::chrono::{FixedOffset, Utc};
use bitwyre_ws_core::JsonSerializable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct TimeData {
    pub unixtime: i64,
    pub rfc3339: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct TimeMessage {
    pub time: TimeData,
}

impl JsonSerializable<'_> for TimeData {}
impl JsonSerializable<'_> for TimeMessage {}

impl Default for TimeMessage {
    fn default() -> Self {
        let current_server_time = Utc::now().with_timezone(&FixedOffset::east(0));
        let current_unixtime = current_server_time.timestamp_nanos();
        let current_rfc3339 = current_server_time.to_rfc3339();
        Self {
            time: TimeData {
                unixtime: current_unixtime,
                rfc3339: current_rfc3339,
            },
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_json_reconstruction() {
        let time_message = TimeMessage::default();
        let time_message_json = time_message.to_json();
        let reconstructed_time_message = TimeMessage::from_json(&time_message_json).unwrap();
        assert_eq!(
            reconstructed_time_message.time.unixtime,
            time_message.time.unixtime
        );
    }
}
