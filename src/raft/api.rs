use super::raft_conf::{ConfigRaft, RaftVar, };
use std::collections::{HashMap, HashSet, VecDeque};
use crate::trans::client::req_post;
use crate::raft::db::*;
use super::raft_enum:: Which;

// data is data to save need export
pub async fn append_entry(data: &str) -> anyhow::Result<String> {
    let leader_url=RaftVar::leader_url();
    let id = req_post(&leader_url, Which::append_entry, data).await?;
    Ok(id)
}

// need export only save id of data ,req data from leader
pub async fn query_id(id: &str) -> anyhow::Result<String> {
    let leader_url=RaftVar::leader_url();
    let data = req_post(&leader_url, Which::query_id, id).await?;
    Ok(data)
}
// need export
pub async fn fetch_ids() -> anyhow::Result<String> {
    let ids=RaftVar::snap_ids();
    let res1= serde_json::to_string(&ids)?;
    Ok(res1)
}