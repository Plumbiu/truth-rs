use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use sonic_rs::to_string;
use std::fs;
use truth_rs_type::RelationQuery;

#[get("/relations.json/query")]
async fn relations_name_json(
    data: web::Data<AppState>,
    query: web::Query<RelationQuery>,
) -> impl Responder {
    let path = &data.relations.get(&query.name).unwrap().path;
    let pkg = fs::read_to_string(path).unwrap();
    HttpResponse::Ok().body(pkg)
}
#[get("/relations.json")]
async fn relations_json(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(to_string(&data.relations).unwrap())
}
