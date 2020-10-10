#[macro_use]
extern crate anyhow;

pub mod trans;
pub mod raft;

pub use raft::raft_conf::ConfigRaft;
pub use trans::server::init_app;
pub use trans::server::run_app;
pub use raft::raft_run::cron_app;