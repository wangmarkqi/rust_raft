use std::sync::mpsc;
use std::{thread, time};
use crate::raft::raft_conf::{CONF, ROLE, LEADER, ConfigRaft};
use super::router::app_router;
use std::sync::{Arc, Mutex};
use crate::raft::raft_enum::{Fields, Role};
use crate::raft::db::insert;

pub fn init_app(conf: ConfigRaft) -> anyhow::Result<()> {
    CONF.set(conf).unwrap();
    let conf = CONF.get().expect("can not get config raft");
    if !conf.first_node {
        return Ok(());
    }
    let role = Arc::clone(&ROLE);
    let mut role = role.lock().unwrap();
    *role=Role::leader.name().to_string();
    // change leader here
    let leader = Arc::clone(&LEADER);
    let mut leader_url = leader.lock().unwrap();
    *leader_url = conf.url_me.clone();

    Ok(())
}

pub fn run_app() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let _ = app_router(tx);
    });

    // let srv = rx.recv().unwrap();
    // println!("WATING 40 SECONDS");
    // thread::sleep(time::Duration::from_secs(40));
    // println!("STOPPING SERVER");
    // rt::System::new("").block_on(srv.stop(true));
}