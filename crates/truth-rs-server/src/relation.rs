use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use simd_json::to_string;

#[get("/relations.json/{name}")]
async fn relations_name_json(data: web::Data<AppState>, name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(to_string(&data.relations.get(&name.as_str().to_string())).unwrap())
}
#[get("/relations.json")]
async fn relations_json(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(to_string(&data.relations).unwrap())
}
