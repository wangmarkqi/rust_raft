use std::collections::HashMap;
use crate::raft::raft_conf::{ConfigRaft};
use crate::raft::raft_enum::{Which};


pub async fn test(conf: &ConfigRaft) {
    let url1 = format!("https://{}/{}/{}", conf.url_me, "user", "wq");
    let res = req_get(&url1).await;
    dbg!(res);
    let res2 = req_post(&conf.url_me,Which::ask_leader,"").await;
    dbg!(res2);
}

pub async fn req_get(url: &str) -> anyhow::Result<String> {
    let cli = reqwest::Client::builder().danger_accept_invalid_certs(true)
        .no_proxy().build()?;
    let resp = cli.get(url).send().await?.text().await?;
    Ok(resp)
}

pub async fn req_post(url: &str, which: Which, data: &str) -> anyhow::Result<String> {
    let url2 = format!("https://{}/", url);

    let mut params = HashMap::new();
    params.insert("which", which.name());
    params.insert("data", data);

    let cli = reqwest::Client::builder().danger_accept_invalid_certs(true)
        .no_proxy().build()?;
    let resp = cli.post(&url2).form(&params).send().await?.text().await?;

    Ok(resp)
}
