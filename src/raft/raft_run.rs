use chrono::prelude::*;
use chrono::Utc;
use super::raft_enum::{Role, Which, Fields};
use std::sync::{Arc, Mutex};
use super::raft_conf::{ROLE, CONF, ConfigRaft};
use std::collections::HashMap;
use crate::trans::client::req_post;
use crate::raft::db::{get, insert};
use super::req::{ask_confirm_leader, ask_find_leader};
use std::time::Duration;
use async_std::task;
use crate::raft::req::{ask_heart_beat, ask_peer_urls, ask_snapshot_ids, ask_peers_vote, find_leader_again, ask_leader_change};

pub async fn cron_app()->anyhow::Result<()> {
    let conf = CONF.get().expect("can not get config raft");
    let tick_hb = conf.heartbeat_tick;
    let tick_vote = conf.election_tick;
    let tick_peer = conf.update_peers_tick;
    let tick_snapshot = conf.update_snapshot_tick;
    let duration = Duration::from_secs(tick_hb as u64);

    let mut last_hb = Local::now().time();
    ask_find_leader(&conf.url_peer).await.unwrap();
    ask_confirm_leader().await.unwrap();

    loop {
        task::sleep(duration).await;
        let now = Local::now().time();
        let differ = now - last_hb;
        let elapse = differ.num_seconds();

        let role = Arc::clone(&ROLE);
        let mut role = role.lock().unwrap();

        if *role == Role::follower.name() {
            let res1 = ask_heart_beat().await?;
            if res1 {
                last_hb = now;
            } else {
                if elapse > tick_vote {
                    *role = Role::candidate.name().to_string();
                    last_hb = now;
                }
            }
            if elapse > tick_peer {
                ask_peer_urls().await?;
                last_hb = now;
            }
            if elapse > tick_snapshot {
                ask_snapshot_ids().await?;
                last_hb = now;
            }
        }

        if *role == Role::candidate.name() {
            let win = ask_peers_vote().await?;
            if win {
                ask_leader_change().await?;
            } else {
                let find=find_leader_again().await?;
                if !find{
                    panic!("last contact from all peers and leader");
                }
            }
        }
    }
}


pub fn time_now_str() -> String {
    let local = Local::now();
    let s = local.to_rfc3339();
    s
}


pub fn time_differ(origin: &str) -> i64 {
    let a = Local::now().time();
    let b = DateTime::parse_from_rfc3339(origin).unwrap();
    let c = b.time();
    let differ = a - c;
    println!("Total time taken to run is {}", differ.num_seconds());
    let res = differ.num_seconds();
    res
}
