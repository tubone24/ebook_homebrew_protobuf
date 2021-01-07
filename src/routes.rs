use actix_web::{web, HttpRequest, HttpResponse, Responder};
use crate::controllers::{index, proto};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index::index));
    cfg.service(web::resource("proto").route(web::post().to(proto::proto)));
}