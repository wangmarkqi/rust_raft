# RUST_RAFT
This crate is aimed to be rust raft  implementation, which are  a key component of many distributed systems. 

This crate guarantee:
- There is one and only one leader in any time;
- Data are synchronized with leader, but only keep latest items no more than snapshot numbers in initial config.
 it is user responsibility to fetch and save data from snapshot, to synchronize data out which is not in the scope of current snapshot.


  ## Quick Start 
  see bin.rs
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
