use chrono::prelude::*;
use chrono::NaiveTime;
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

    let mut timer_vote = Timer::init(conf.election_tick);
    let mut timer_peer = Timer::init(conf.update_peers_tick);
    let mut timer_snapshot = Timer::init(conf.update_snapshot_tick);
    let mut timer_clear = Timer::init(conf.clear_db_tick);

    if !conf.first_node {
        ask_find_leader(&conf.url_peer).await.unwrap();
        ask_confirm_leader().await.unwrap();
    }

    loop {
        tokio::time::delay_for(duration).await;
        dbg!("round anaig");

        let role = RaftVar::role();

        if role == Role::follower.name().to_string() {
            let rec_hb = ask_heart_beat().await?;
            if rec_hb {
                timer_vote.sleep_again();
            } else {
                if timer_vote.wake_up() {
                    RaftVar::set_role(Role::candidate.name());
                    timer_vote.sleep_again();
                }
            }
            if timer_peer.wake_up() {
                ask_peer_urls().await?;
                timer_peer.sleep_again();
            }
            if timer_snapshot.wake_up() {
                ask_snapshot_ids().await?;
                timer_snapshot.sleep_again();
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
                RaftVar::set_role(Role::follower.name());
                timer_vote.sleep_again();
            }
        }
        if role == Role::leader.name().to_string() {
            if timer_clear.wake_up() {
                clear_db()?;
                timer_clear.sleep_again();
            }
        }
    }
}


fn clear_db() -> anyhow::Result<()> {
    let ids = RaftVar::snap_ids();
    for kv in DB.iter() {
        let res = kv?.0.to_vec();
        let k = String::from_utf8(res)?;
        if !ids.contains(&k) && k != Fields::peer_urls.name().to_string() {
            remove(&k)?;
        }
    }
    Ok(())
}


struct Timer {
    ticks: i64,
    last: NativeTime,
}

impl Timer {
    fn init(tick: i64) -> Self {
        Timer {
            ticks: tick,
            last: Local::now().time(),
        }
    }
    fn wake_up(&self) -> bool {
        let now = Local::now().time();
        let differ = now - self.last;
        let elapse = differ.num_seconds();
        elapse > self.ticks
    }
    fn sleep_again(&mut self) {
        let now = Local::now().time();
        self.last = now;
    }
}