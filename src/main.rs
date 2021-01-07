use actix_web::{App, HttpServer, middleware, web};
use actix_grpc::routes;
use actix_protobuf::*;
use prost_derive::Message;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=debug");
    env_logger::init();
    HttpServer::new(|| App::new().wrap(middleware::Logger::default()).configure(routes::routes))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}