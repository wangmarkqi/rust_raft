use actix_web::{ web, App, HttpRequest, HttpResponse, HttpServer};



pub async fn with_param(
    req: HttpRequest,
    web::Path((name, )): web::Path<(String, )>,
) -> HttpResponse {
    println!("{:?}", req);

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Hello {}!", name))
}