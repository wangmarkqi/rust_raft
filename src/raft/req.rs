use super::raft_conf::{ConfigRaft, RaftVar,CONF};
use std::collections::{HashMap, HashSet,VecDeque};
use crate::trans::client::req_post;
use super::raft_enum::{Role, Which, Fields};
use std::sync::{Arc, Mutex};
use super::db::*;

// befor loop and after election fail
pub async fn ask_find_leader(url: &str) -> anyhow::Result<()> {
    let res = req_post(url, Which::ask_leader, "").await?;
    // change leader here
    RaftVar::set_leader_url(&res);
    Ok(())
}

// befor loop
pub async fn ask_confirm_leader() -> anyhow::Result<bool> {
    let leader=RaftVar::leader_url();

    let res = req_post(&leader, Which::confirm_leader, "").await?;
    if res == Role::leader.name() {
        return Ok(true);
    }
    Ok(false)
}

// high frequency
pub async fn ask_heart_beat() -> anyhow::Result<bool> {
    let leader_url=RaftVar::leader_url();
    if leader_url == "".to_string() {
        panic!("leader url is empty")
    }
    let res = req_post(&leader_url, Which::heart_beat, "").await?;
    if res == Fields::success.name() {
        return Ok(true);
    }
    Ok(false)
}

// low frequency,report my url and update peer urls
pub async fn ask_peer_urls() -> anyhow::Result<bool> {
    let leader_url=RaftVar::leader_url();
    let conf = CONF.get().expect("can not get config raft");
    let me = &conf.url_me;
    let res = req_post(&leader_url, Which::peer_urls, me).await?;
    let res1: HashSet<String> = serde_json::from_str(&res)?;
    replace_set_from_set(Fields::peer_urls.name(), &res1)?;
    del_str_in_set(Fields::peer_urls.name(), &conf.url_me)?;
    Ok(true)
}
pub async fn ask_snapshot_ids() -> anyhow::Result<()> {
    let leader_url=RaftVar::leader_url();
    let res = req_post(&leader_url, Which::snapshot_ids, "").await?;
    let res1: VecDeque<String> = serde_json::from_str(&res)?;
    RaftVar::replace_snap_ids(res1);
    Ok(())
}

// low frequency,check all url and del dead one,result will be success,fail ,none(err dead)
pub async fn ask_peers_vote() -> anyhow::Result<bool> {
    let snap=RaftVar::snap_ids();
    let snapshot = serde_json::to_string(&snap)?;

    let _peers = get(Fields::peer_urls.name())?;
    let peers: HashSet<String> = serde_json::from_str(&_peers)?;

    let mut score = HashMap::new();

    for peer in peers.iter() {
        let res = req_post(peer, Which::peer_vote, &snapshot).await;
        match res {
            Ok(s) => score.insert(peer.to_string(), s),
            Err(e) => score.insert(peer.to_string(), Fields::none.name().to_string()),
        };
    }
    check_i_am_leader(&score)
}

fn check_i_am_leader(score: &HashMap<String, String>) -> anyhow::Result<bool> {
    let mut peers = HashSet::new();
    let mut agree = 0;
    let mut reject = 0;
    for (k, v) in score.iter() {
        if v == Fields::success.name() {
            peers.insert(k.to_string());
            agree = agree + 1;
        }
        if v == Fields::fail.name() {
            peers.insert(k.to_string());
            reject = reject + 1;
        }
    }
    if peers.len() > 0 {
        replace_set_from_set(Fields::peer_urls.name(), &peers)?;
    }
    if agree >= reject {
        return Ok(true);
    }
    Ok(false)
}

// if not find should panic
pub async fn find_leader_again() -> anyhow::Result<bool> {
    let peers = get(Fields::peer_urls.name())?;
    let peers: HashSet<String> = serde_json::from_str(&peers)?;
    for peer in peers.iter() {
        ask_find_leader(peer).await?;
        let find = ask_confirm_leader().await?;
        if find {
            RaftVar::set_role(Role::follower.name());
            return Ok(true);
        }
    }
    Ok(false)
}

pub async fn ask_leader_change() -> anyhow::Result<()> {
    let conf = CONF.get().expect("can not get config raft");
    let _peers = get(Fields::peer_urls.name())?;
    let peers: HashSet<String> = serde_json::from_str(&_peers)?;
    for peer in peers.iter() {
        req_post(peer, Which::leader_change, &conf.url_me).await?;
    }
    Ok(())
}
