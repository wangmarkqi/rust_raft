# RUST_RAFT
This crate is aimed to be rust raft  implementation, which are  a key component of many distributed systems. 

This crate guarantee:
- There is one and only one leader in any time;
- Data are synchronized with leader, but only keep latest items no more than snapshot numbers in initial config.
 
 It is user responsibility to fetch data from snapshot, to save data to database, and to synchronize data  which is not in the scope of current snapshot.


  ## Quick Start 
  - This is normal process to start raft.
  Git clone the crate and run bin.rs.
```
pub mod trans;
pub mod raft;

#[macro_use]
extern crate anyhow;

#[tokio::main]
async fn main() {
    let conf = raft::raft_conf::ConfigRaft::default();
    trans::server::init_app(conf);
    trans::server::run_app();
    raft::raft_run::cron_app().await;
}

```
 - api to process data:
 
    - append_entry: user call this funciton when add data to distribution system.
    
    - fetch_ids: use call this function to synchronize snapshot with leader. cron_app() function will call this function every ConfigRaft.update_snapshot_tick.
    
    - query_id: use id to query data from leader,user should save the data to database.It is recommended to save id in the same time to judge duplicate.