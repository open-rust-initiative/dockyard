use actix_web::{get, web, HttpResponse, Responder};

use crate::v2::pull::{get_blobs, get_manifests, head_manifests};
use crate::v2::push::{
    head_blobs, patch_uploads_blobs, post_uploads_blobs, put_uploads_blobs, put_uploads_manifests,
};
use crate::v2::tags_list::get_tags_list;
use crate::v2::teardown::{delete_blobs, delete_manifests};
use crate::Calim;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_base);
    cfg.service(head_blobs);
    cfg.service(post_uploads_blobs);
    cfg.service(patch_uploads_blobs);
    cfg.service(put_uploads_blobs);
    cfg.service(put_uploads_manifests);
    cfg.service(head_manifests);
    cfg.service(get_manifests);
    cfg.service(get_blobs);
    cfg.service(delete_blobs);
    cfg.service(delete_manifests);
    cfg.service(get_tags_list);
}
#[get("/")]
async fn get_base(_cal: Calim) -> impl Responder {
    HttpResponse::Ok().finish()
}
