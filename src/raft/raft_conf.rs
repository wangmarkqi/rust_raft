use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use once_cell::sync::{Lazy, OnceCell};
use std::sync::{Arc, Mutex};
use super::raft_enum::{Role};

pub static LEADER: Lazy<Arc<Mutex<String>>> = Lazy::new(|| {
    let res1 = Mutex::new("".to_string());
    let res2 = Arc::new(res1);
    res2
});

pub static ROLE: Lazy<Arc<Mutex<String>>> = Lazy::new(|| {
    let res1 = Mutex::new(Role::follower.name().to_string());
    let res2 = Arc::new(res1);
    res2
});
pub static CONF: OnceCell<ConfigRaft> = OnceCell::new();

#[derive(Debug, Clone)]
pub struct ConfigRaft {
    pub election_tick: i64,
    pub heartbeat_tick: i64,
    pub update_peers_tick: i64,
    pub update_snapshot_tick: i64,
    // 找领导，和领导对peers ，对最后一个区块
    pub snapshot_numbers: i64,
    pub first_node: bool,
    pub url_me: String,
    pub url_peer: String,
    pub tls_cert: String,
    pub tls_key: String,
}

impl ConfigRaft {
    pub fn default() -> ConfigRaft {
        ConfigRaft {
            heartbeat_tick: 1,
            election_tick: 20,
            update_peers_tick: 2,
            update_snapshot_tick: 2,
            snapshot_numbers: 100,
            first_node: true,
            url_me: "127.0.0.1:8442".to_string(),
            url_peer: "127.0.0.1:8442".to_string(),
            tls_cert: "./data/tls/cert.pem".to_string(),
            tls_key: "./data/tls/key.pem".to_string(),
        }
    }
}

