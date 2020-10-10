use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use crate::raft::raft_enum::Which;
use crate::raft::resp::{resp_find_leader, resp_confirm_leader, resp_heart_beat, resp_peer_urls};

#[derive(Serialize, Deserialize)]
pub struct Msg {
    which: Which,
    data: String,
}

pub async fn handle_post(params: web::Form<Msg>) -> actix_web::Result<HttpResponse> {
    let which = &params.which;
    let data = &params.data;
    let res = dispatch_post(which, data).await;

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(res))
}

async fn dispatch_post(which: &Which, data: &str) -> String {
    let res = match which {
        Which::ask_leader => resp_find_leader().await,
        Which::confirm_leader => resp_confirm_leader().await,
        Which::heart_beat => resp_heart_beat().await,
        Which::peer_urls => resp_peer_urls(data).await,
        _ => Err(anyhow!("not match")),
    };
    match res {
        Ok(s) => s,
        Err(e) => {
            let res = format!("raft error from leader: {}", e);
            res
        }
    }
}