use std::collections::{HashMap, HashSet};
use std::fs;
use once_cell::sync::Lazy;

pub static DB: Lazy<sled::Db> = Lazy::new(|| {
    let dir = "./data/db";
    fs::create_dir_all(dir).unwrap();
    sled::open(dir).unwrap()
});

pub fn insert(k: &str, v: &str) -> anyhow::Result<()> {
    let kk = k.as_bytes().to_vec();
    let vv = v.as_bytes().to_vec();
    DB.insert(kk, vv)?;
    Ok(())
}

pub fn contain(k: &str) -> bool{
    let kk = k.as_bytes().to_vec();
    let existed= DB.contains_key(kk.clone());
    match existed{
        Ok(yes)=>yes,
        Err(e)=>false,
    }
}
// if k not return""
pub fn get(k: &str) -> anyhow::Result<String> {
    if !contain(k){
       return Ok("".to_string());
    }
    let res1 = DB.get(&k).expect("can not read db");
    if let Some(res2) = res1 {
        let mut res3 = vec![];
        for i in res2.iter() {
            res3.push(*i);
        }
        let res4 = String::from_utf8(res3)?;
        return Ok(res4);
    }
    Ok("".to_string())
}

pub fn remove(k: &str) -> anyhow::Result<()> {
    if contain(k){
        let kk = k.as_bytes().to_vec();
        DB.remove(kk)?;
    }
    Ok(())
}

// if k not exist or v empty ,return []
pub fn read_set(dbk: &str) -> anyhow::Result<HashSet<String>> {
    let s = {
        let a = get(dbk)?;
        let res1 = serde_json::from_str(&a);
        if let Ok(res2) = res1 {
            res2
        } else {
            let res3: HashSet<String> = HashSet::new();
            res3
        }
    };
    Ok(s)
}

pub fn update_set_from_str(dbk: &str, content: &str) -> anyhow::Result<String> {
    let mut peers = read_set(dbk)?;
    let c = content.trim().to_string();
    if c != "" {
        peers.insert(c);
    }
    let l_str = serde_json::to_string(&peers)?;
    insert(dbk, &l_str)?;
    Ok(l_str)

}


pub fn update_set_from_set(dbk: &str, l: &HashSet<String>) -> anyhow::Result<String> {
    let mut peers = read_set(dbk)?;
    for content in l {
        let c = content.trim().to_string();
        if c != "" {
            peers.insert(c);
        }
    }
    let l_str = serde_json::to_string(&peers)?;
    insert(dbk, &l_str)?;
    Ok(l_str)
}

pub fn replace_set_from_set(dbk: &str, l: &HashSet<String>) -> anyhow::Result<String> {
    remove(dbk)?;
    update_set_from_set(dbk, l)
}

pub fn del_str_in_set(dbk: &str, e: &str) -> anyhow::Result<()> {
    let mut l = read_set(dbk)?;
    l.retain(|x| x != e);
    let js = serde_json::to_string(&l)?;
    insert(dbk, &js)
}