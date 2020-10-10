use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc;
use actix_web::{dev::Server,rt, web, App, HttpRequest, HttpResponse, HttpServer};
use rustls::internal::pemfile::{certs, rsa_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use serde::{Deserialize, Serialize};
use crate::raft::raft_conf::{CONF,ConfigRaft};
use super::handle_post::handle_post;
use super::handle_get::with_param;

pub fn app_router(tx: mpsc::Sender<Server>) -> std::io::Result<()> {
    let mut sys = rt::System::new("raft");

    let conf=CONF.get().expect("can not get config raft");
    let config=config_server();
    let srv=HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::post().to(handle_post)))
            .service(web::resource("/user/{name}").route(web::get().to(with_param)))
    })
        .bind_rustls(&conf.url_me, config)?
        .run();

    // send server controller to main thread
    let _ = tx.send(srv.clone());

    // run future
    sys.block_on(srv)
}



fn config_server() -> rustls::ServerConfig {
    let conf=CONF.get().expect("can not get config raft");
    // load ssl keys
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open(&conf.tls_cert).unwrap());
    let key_file = &mut BufReader::new(File::open(&conf.tls_key).unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = rsa_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();
    config
}




