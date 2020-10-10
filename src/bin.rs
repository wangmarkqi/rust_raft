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

