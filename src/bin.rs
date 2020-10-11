pub mod trans;
pub mod raft;
use crate::raft::db::*;

#[macro_use]
extern crate anyhow;


#[tokio::main]
async fn main() ->anyhow::Result<()>{
    // let conf=raft::raft_conf::ConfigRaft::default();
    // trans::server::init_app(conf);
    // trans::server::run_app();
    // raft::raft_run::cron_app().await;
    insert("wq","ad");
    for kv in DB.iter() {
        let res2 = kv?.0.to_vec();
        dbg!(String::from_utf8(res2));
    }
    Ok(())



}

