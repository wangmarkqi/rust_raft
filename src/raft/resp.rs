use crate::trans::client::req_post;
use crate::raft::db::*;
use super::raft_enum::{Role, Which, Fields};
use super::raft_conf::{ConfigRaft, CONF, RaftVar};
use std::sync::{Arc, Mutex};
use std::collections::{HashMap, HashSet, VecDeque};
use super::req::ask_confirm_leader;

pub async fn resp_find_leader() -> anyhow::Result<String> {
    let leader_url=RaftVar::leader_url();
    if leader_url == "".to_string() {
        return Err(anyhow!("the leader url of mine is emtpy too"));
    }
    Ok(leader_url)
}


pub async fn resp_confirm_leader() -> anyhow::Result<String> {
    Ok(RaftVar::role())
}

pub async fn resp_heart_beat() -> anyhow::Result<String> {
    let role = RaftVar::role();
    if role == Role::leader.name().to_string() {
        return Ok(Fields::success.name().to_string());
    }
    Ok(Fields::fail.name().to_string())
}

// save url inclue leader
pub async fn resp_peer_urls(url_peer: &str) -> anyhow::Result<String> {
    let conf = CONF.get().expect("can not get config raft");
    let mut s = HashSet::new();
    s.insert(url_peer.to_string());
    s.insert(conf.url_me.to_string());
    let res = update_set_from_set(Fields::peer_urls.name(), &s)?;
    Ok(res)
}

pub async fn resp_append_entry(data: &str) -> anyhow::Result<String> {
    let now = chrono::Local::now();
    let id = now.timestamp_nanos().to_string();
    RaftVar::add_snap_ids(&id);
    insert(&id, data)?;
    Ok(id)
}

pub async fn resp_query_id(id: &str) -> anyhow::Result<String> {
    let res = get(id)?;
    Ok(res)
}

pub async fn resp_snapshot_ids() -> anyhow::Result<String> {
    let ids=RaftVar::snap_ids();
    let res = serde_json::to_string(&ids)?;
    Ok(res)
}

pub async fn resp_peers_vote(snapshot: &str) -> anyhow::Result<String> {
    let role = RaftVar::role();
// if i am leader ,return false
    if role == Role::leader.name().to_string() {
        return Ok(Fields::fail.name().to_string());
    }
    // if current leader confirm fail
    let leader_exist = ask_confirm_leader().await?;
    if leader_exist {
        return Ok(Fields::fail.name().to_string());
    }
    // if yours contains mine
    let better = yours_snapshot_better(snapshot)?;
    if !better {
        return Ok(Fields::fail.name().to_string());
    }
    Ok(Fields::success.name().to_string())
}

fn yours_snapshot_better(snapshot: &str) -> anyhow::Result<bool> {
    let mine=RaftVar::snap_ids();
    let yours:VecDeque<String> = serde_json::from_str(&snapshot)?;
    for my in mine.iter() {
        if !yours.contains(my) {
            return Ok(false);
        }
    }
    Ok(true)
}

pub async fn resp_leader_change(leader_url: &str) -> anyhow::Result<String> {
    let role=RaftVar::role();
// if i am not follower
    if role != Role::follower.name().to_string() {
        RaftVar::set_role(Role::follower.name());
    }
    // change leader here
    RaftVar::set_leader_url(leader_url);
    Ok(Fields::success.name().to_string())
}
