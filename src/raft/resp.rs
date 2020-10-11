use crate::trans::client::req_post;
use crate::raft::db::*;
use super::raft_enum::{Role, Which, Fields};
use super::raft_conf::{ConfigRaft, CONF, RV};
use std::sync::{Arc, Mutex};
use std::collections::{HashMap, HashSet};
use super::req::ask_confirm_leader;

pub async fn resp_find_leader() -> anyhow::Result<String> {
    let leader = Arc::clone(&LEADER);
    let leader_url = leader.lock().unwrap();
    if *leader_url == "".to_string() {
        return Err(anyhow!("the leader url of mine is emtpy too"));
    }
    Ok(leader_url.to_string())
}


pub async fn resp_confirm_leader() -> anyhow::Result<String> {
    let role = Arc::clone(&ROLE);
    let role = role.lock().unwrap();
    Ok(role.to_string())
}

pub async fn resp_heart_beat() -> anyhow::Result<String> {
    let role = Arc::clone(&ROLE);
    let role = role.lock().unwrap();
    if *role == Role::leader.name() {
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
    update_set_from_str(Fields::snapshots_ids.name(), &id)?;
    insert(&id, data)?;
    Ok(id)
}

pub async fn resp_query_id(id: &str) -> anyhow::Result<String> {
    let res = get(id)?;
    Ok(res)
}

pub async fn resp_snapshot_ids() -> anyhow::Result<String> {
    let ids = read_set(Fields::snapshots_ids.name())?;
    let res = serde_json::to_string(&ids)?;
    Ok(res)
}

pub async fn resp_peers_vote(snapshot: &str) -> anyhow::Result<String> {
    let role = Arc::clone(&ROLE);
    let role = role.lock().unwrap();
// if i am leader ,return false
    if *role == Role::leader.name() {
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
    let mine = get(Fields::snapshots_ids.name())?;
    let mine: HashSet<String> = serde_json::from_str(&mine)?;
    let yours: HashSet<String> = serde_json::from_str(&snapshot)?;
    for my in mine.iter() {
        if !yours.contains(my) {
            return Ok(false);
        }
    }
    Ok(true)
}

pub async fn resp_leader_change(leader_url: &str) -> anyhow::Result<String> {
    let role = Arc::clone(&ROLE);
    let mut role = role.lock().unwrap();
// if i am not follower
    if *role != Role::follower.name() {
        *role = Role::follower.name().to_string();
    }
    // change leader here
    let leader = Arc::clone(&LEADER);
    let mut url = leader.lock().unwrap();
    *url = leader_url.to_string();
    Ok(Fields::fail.name().to_string())
}
