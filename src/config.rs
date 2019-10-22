use bitwyre_ws_core::get_executable_name;
use bitwyre_ws_core::get_mandatory_env_int;
use bitwyre_ws_core::get_mandatory_env_string;
use std::time::Duration;

const ENV_MAX_CLIENT: &str = "MAX_CLIENT";
const ENV_BLAST_INTERVAL_MS: &str = "BLAST_INTERVAL_MS";
const ENV_SERVICE_IP: &str = "SERVICE_IP";
const ENV_SERVICE_PORT: &str = "SERVICE_PORT";
const ENV_SERVICE_PATH: &str = "SERVICE_PATH";
const ENV_PING_LIMIT_MS: &str = "PING_LIMIT_MS";

#[cfg(debug_assertions)]
fn is_debugging() -> bool {
    true
}

#[cfg(not(debug_assertions))]
fn is_debugging() -> bool {
    false
}

#[derive(Clone)]
pub struct ServiceConfig {
    pub debug_build: bool,
    pub executable_name: String,
    pub max_clients: u32,
    pub blast_interval: Duration,
    pub service_baseurl: String,
    pub service_path: String,
    pub ping_limit_duration: Duration,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        let max_clients = get_mandatory_env_int(ENV_MAX_CLIENT) as u32;
        let blast_interval =
            Duration::from_millis(get_mandatory_env_int(ENV_BLAST_INTERVAL_MS) as u64);
        let service_baseurl = format!(
            "{}:{}",
            get_mandatory_env_string(ENV_SERVICE_IP),
            get_mandatory_env_int(ENV_SERVICE_PORT)
        );
        let service_path = get_mandatory_env_string(ENV_SERVICE_PATH);
        let ping_limit_duration =
            Duration::from_millis(get_mandatory_env_int(ENV_PING_LIMIT_MS) as u64);
        Self {
            debug_build: is_debugging(),
            executable_name: get_executable_name(),
            max_clients,
            blast_interval,
            service_baseurl,
            service_path,
            ping_limit_duration,
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_creation_is_successful() {
        let max_clients: u32 = 2000;
        let blast_interval_ms: u64 = 1000;
        let ping_limit_ms: u64 = 1000;
        let bind_ip = "127.0.0.1";
        let bind_port = 7000;
        let bind_path = "/public/time";
        let bind_baseurl = format!("{}:{}", bind_ip, bind_port);
        let blast_interval = Duration::from_millis(blast_interval_ms);
        let ping_limit_duration = Duration::from_millis(ping_limit_ms);
        env::set_var(ENV_MAX_CLIENT, max_clients.to_string());
        env::set_var(ENV_BLAST_INTERVAL_MS, blast_interval_ms.to_string());
        env::set_var(ENV_SERVICE_IP, bind_ip);
        env::set_var(ENV_SERVICE_PORT, bind_port.to_string());
        env::set_var(ENV_SERVICE_PATH, bind_path);
        env::set_var(ENV_PING_LIMIT_MS, ping_limit_ms.to_string());
        let config: ServiceConfig = Default::default();
        assert_eq!(config.max_clients, max_clients);
        assert_eq!(config.blast_interval, blast_interval);
        assert_eq!(config.service_baseurl, bind_baseurl);
        assert_eq!(config.service_path, bind_path);
        assert_eq!(config.ping_limit_duration, ping_limit_duration);
    }
}
