use chrono::prelude::*;
use chrono::Utc;
use super::raft_enum::{Role, Which, Fields};
use std::sync::{Arc, Mutex};
use super::raft_conf::{CONF, RV, ConfigRaft};
use std::collections::HashMap;
use crate::trans::client::req_post;
use crate::raft::db::{get, DB, remove, insert};
use super::req::{ask_confirm_leader, ask_find_leader};
use std::time::Duration;
use crate::raft::req::{ask_heart_beat, ask_peer_urls, ask_peers_vote, find_leader_again, ask_snapshot_ids, ask_leader_change};
use crate::raft::raft_conf::RaftVar;

pub async fn cron_app() -> anyhow::Result<()> {
    let conf = CONF.get().expect("can not get config raft");
    let tick_hb = conf.heartbeat_tick;
    let duration = Duration::from_secs(tick_hb as u64);

    let tick_vote = conf.election_tick;
    let tick_peer = conf.update_peers_tick;
    let tick_snapshot = conf.update_snapshot_tick;
    let tick_clear = conf.clear_db_tick;

    let mut last_hb = Local::now().time();
    if !conf.first_node {
        ask_find_leader(&conf.url_peer).await.unwrap();
        ask_confirm_leader().await.unwrap();
    }

    loop {
        tokio::time::delay_for(duration).await;
        dbg!("round anaig");
        let now = Local::now().time();
        let differ = now - last_hb;
        let elapse = differ.num_seconds();

        let role = RaftVar::role();

        if role == Role::follower.name().to_string() {
            let res1 = ask_heart_beat().await?;
            if res1 {
                last_hb = now;
            } else {
                if elapse > tick_vote {
                    RaftVar::set_role(Role::candidate.name());
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

        if role == Role::candidate.name().to_string() {
            let win = ask_peers_vote().await?;
            if win {
                ask_leader_change().await?;
            } else {
                let find = find_leader_again().await?;
                if !find {
                    panic!("last contact from all peers and leader");
                }
            }
        }
        if role == Role::leader.name().to_string() {
            if elapse>tick_clear{
                clear_db()?;
            }

        }
    }
}


fn clear_db()  ->anyhow::Result<()>{
    let ids = RaftVar::snap_ids();
    for kv in DB.iter() {
        let res = kv?.0.to_vec();
        let k=String::from_utf8(res)?;
        if !ids.contains(&k) && k!=Fields::peer_urls.name().to_string(){
            remove(&k);
        }
    }
    Ok(())
}


