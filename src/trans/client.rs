use std::collections::HashMap;
use crate::raft::raft_conf::{ConfigRaft,CONF};
use crate::raft::raft_enum::{Which};




pub async fn req_get(url: &str) -> anyhow::Result<String> {
    let conf=CONF.get().expect("can not find conf");
    let me=&conf.url_me;
    if me==url{
        return Err(anyhow!("can not get to self"));
    }
    let cli = reqwest::Client::builder().danger_accept_invalid_certs(true)
        .no_proxy().build()?;
    let resp = cli.get(url).send().await?.text().await?;
    Ok(resp)
}

pub async fn req_post(url: &str, which: Which, data: &str) -> anyhow::Result<String> {
    let conf=CONF.get().expect("can not find conf");
    let me=&conf.url_me;
    if me==url{
        return Err(anyhow!("can not post to self"));
    }
    let url2 = format!("https://{}/", url);

    let mut params = HashMap::new();
    params.insert("which", which.name());
    params.insert("data", data);

    let cli = reqwest::Client::builder().danger_accept_invalid_certs(true)
        .no_proxy().build()?;
    let resp = cli.post(&url2).form(&params).send().await?.text().await?;

    Ok(resp)
}
