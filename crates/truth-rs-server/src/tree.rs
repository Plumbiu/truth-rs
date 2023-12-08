use crate::AppState;
use actix_web::get;
use actix_web::{web, HttpResponse, Responder};
use truth_rs_core::tree::stringify_tree;

#[get("/tree.json")]
async fn tree_json(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(stringify_tree(&data.relations, 3))
}
