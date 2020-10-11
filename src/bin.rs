// pub mod trans;
pub mod raft;
use std::sync::{Arc, Mutex};
#[macro_use]
extern crate anyhow;
use raft::raft_conf::{RV,CONF,ConfigRaft};
use crate::raft::raft_conf::RaftVar;

#[tokio::main]
async fn main() {
    // trans::server::init_app(conf);
    // trans::server::run_app();
    // raft::raft_run::cron_app().await;

    test1();
    let rv=RaftVar::role();
    dbg!(rv);

}
fn test1(){
    let conf = ConfigRaft::default();
    CONF.set(conf).unwrap();
    let conf=CONF.get().expect("asd");
    let rv= Arc::clone(&RV);
    let mut rv= rv.lock().unwrap();
    rv.leader_url="3334444444fas".to_string();
}

