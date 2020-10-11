use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use once_cell::sync::{Lazy, OnceCell};
use std::sync::{Arc, Mutex};
use super::raft_enum::{Role};
use std::collections::VecDeque;

pub static CONF: OnceCell<ConfigRaft> = OnceCell::new();
#[derive(Debug, Clone)]
pub struct ConfigRaft {
    pub election_tick: i64,
    pub heartbeat_tick: i64,
    pub update_peers_tick: i64,
    pub update_snapshot_tick: i64,
    pub clear_db_tick:i64,
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
            election_tick: 10,
            update_peers_tick: 10,
            update_snapshot_tick: 10,
            clear_db_tick: 1000,
            snapshot_numbers: 1000,
            first_node: true,
            url_me: "127.0.0.1:8442".to_string(),
            url_peer: "127.0.0.1:8442".to_string(),
            tls_cert: "./data/tls/cert.pem".to_string(),
            tls_key: "./data/tls/key.pem".to_string(),
        }
    }
}




pub static RV: Lazy<Arc<Mutex<RaftVar>>> = Lazy::new(|| {
    let rv=RaftVar::default();
    let res1 = Mutex::new(rv);
    let res2 = Arc::new(res1);
    res2
});

#[derive(Debug, Clone)]
pub struct RaftVar {
    pub role: String,
    pub leader_url: String,
    pub snap_ids: VecDeque<String>,
    pub snap_size:usize,
}

impl RaftVar {
    pub fn default() -> Self {
        let conf = CONF.get().expect("can not get config raft");
        let size = conf.snapshot_numbers as usize;
        let buf: VecDeque<String> = VecDeque::with_capacity(size );
        RaftVar {
            role: Role::follower.name().to_string(),
            leader_url: "".to_string(),
            snap_ids:buf,
            snap_size:size,
        }
    }
    pub fn role()->String{
        let rv= Arc::clone(&RV);
        let  rv= rv.lock().unwrap();
        rv.role.clone()
    }
    pub fn snap_ids()->VecDeque<String>{
        let rv= Arc::clone(&RV);
        let  rv= rv.lock().unwrap();
        rv.snap_ids.clone()
    }

    pub fn leader_url()->String{
        let rv= Arc::clone(&RV);
        let  mut rv= rv.lock().unwrap();
        rv.leader_url.clone()
    }
    pub fn set_role(role:&str){
        let rv= Arc::clone(&RV);
        let mut rv= rv.lock().unwrap();
        rv.role=role.to_string();
    }
    pub fn add_snap_ids(id:&str){
        let rv= Arc::clone(&RV);
        let mut rv= rv.lock().unwrap();
        let size=rv.snap_size;
        rv.snap_ids.push_front(id.to_string());
        rv.snap_ids.truncate(size);
    }
    pub fn replace_snap_ids(mut ids: VecDeque<String>){
        let rv= Arc::clone(&RV);
        let mut rv= rv.lock().unwrap();
        let size=rv.snap_size;
        ids.truncate(size);
        rv.snap_ids=ids;
    }
    pub fn set_leader_url(url:&str){
        let rv= Arc::clone(&RV);
        let mut rv= rv.lock().unwrap();
        rv.leader_url=url.to_string();
    }
}

