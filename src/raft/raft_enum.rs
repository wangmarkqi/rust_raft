use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[macro_export]
macro_rules! enum_str {
    (pub enum $name:ident {
        $($variant:ident ),*,
    }) => {
        #[derive(Serialize, Deserialize)]
        pub enum $name {
            $($variant ),*
        }

        impl $name {
            pub fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };
}
enum_str! {
    pub enum Role {
        leader,
        candidate,
        follower,
    }
}



enum_str! {
    pub enum Which{
        ask_leader,
        confirm_leader,
        heart_beat,
        peer_urls,
        peer_vote,
        leader_change,
        snapshot_ids,
        append_entry,
        send_data,
    }
}
enum_str! {
    pub enum Fields{
        snapshots_ids,
        peer_urls,
        success,
        fail,
        none,
    }
}
