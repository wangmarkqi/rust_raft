use super::raft_conf::{ConfigRaft, CONF, RV, };
use std::collections::{HashMap, HashSet};
use crate::trans::client::req_post;
use crate::raft::db::*;
use super::raft_enum::{Role, Which, Fields};
use std::sync::{Arc, Mutex};

// data is data to save need export
pub async fn ask_append_entry(data: &str) -> anyhow::Result<String> {
    let leader = Arc::clone(&LEADER);
    let leader_url = leader.lock().unwrap();
    let id = req_post(&leader_url, Which::append_entry, data).await?;
    Ok(id)
}

// need export only save id of data ,req data from leader
pub async fn ask_query_id(id: &str) -> anyhow::Result<String> {
    let leader = Arc::clone(&LEADER);
    let leader_url = leader.lock().unwrap();
    let data = req_post(&leader_url, Which::query_id, id).await?;
    Ok(data)
}
// need export
pub async fn ask_snapshot_ids() -> anyhow::Result<HashSet<String>> {
    let leader = Arc::clone(&LEADER);
    let leader_url = leader.lock().unwrap();
    let res = req_post(&leader_url, Which::snapshot_ids, "").await?;
    let res1: HashSet<String> = serde_json::from_str(&res)?;
    replace_set_from_set(Fields::snapshots_ids.name(), &res1)?;
    Ok(res1)
}