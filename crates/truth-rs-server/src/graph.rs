use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use serde_json::to_string;
use std::collections::HashMap;
use truth_rs_core::graph::gen_graph;
use truth_rs_type::RelationsMap;

#[get("/graph.json")]
async fn graph_json(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(to_string(&gen_graph(&data.relations)).unwrap())
}

#[get("/graph.json/{name}")]
async fn graph_name_json(data: web::Data<AppState>, name: web::Path<String>) -> impl Responder {
    let mut map: RelationsMap = HashMap::default();
    let name = name.as_str().to_string();
    let data = match data.relations.get(&name.to_owned()) {
        Some(rel) => {
            map.insert(name, rel.clone());
            HttpResponse::Ok().body(to_string(&gen_graph(&map)).unwrap())
        }
        _ => HttpResponse::Ok().body("null"),
    };
    data
}
