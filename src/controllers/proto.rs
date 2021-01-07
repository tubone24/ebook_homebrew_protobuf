
use actix_web::{web, HttpRequest, HttpResponse, Responder};

use actix_protobuf::*;
use prost_derive::Message;

#[derive(Clone, PartialEq, Message)]
pub struct UploadImage {
    #[prost(string, tag = "1")]
    pub content_type: String,

    #[prost(string, tag = "2")]
    pub name: String,

    #[prost(bytes, tag = "3")]
    pub image: Vec<u8>,
}

#[derive(Clone, PartialEq, Message)]
pub struct UploadImages {
    #[prost(message, repeated,  tag = "1")]
    pub images: Vec<UploadImage>,
}

#[derive(Clone, PartialEq, Message)]
pub struct UploadImageResponse {
    #[prost(bool, tag = "1")]
    pub error: bool,

    #[prost(string, tag = "2")]
    pub message: String,
}

pub async fn proto(msg: ProtoBuf<UploadImages>) -> impl Responder {
    println!("contentType: {:?}", msg.images[0].content_type);
    println!("name: {:?}", msg.images[0].name);
    let resp_obj = UploadImageResponse {error: false, message: "OK".to_string()};
    HttpResponse::Ok().protobuf(resp_obj)
}
