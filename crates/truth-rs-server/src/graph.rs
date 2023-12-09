use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use serde_json::to_string;
use std::collections::HashMap;
use truth_rs_core::graph::gen_graph;
use truth_rs_type::{graph_server::GraphQuery, RelationsMap};

#[get("/graph.json/text")]
async fn graph_json(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(to_string(&gen_graph(&data.relations, None)).unwrap())
}

#[get("/graph.json")]
async fn graph_name_json(
    data: web::Data<AppState>,
    query: web::Query<GraphQuery>,
) -> impl Responder {
    let GraphQuery { name, category } = query.into_inner();
    let data = match data.relations.get(&name.to_owned()) {
        Some(rel) => {
            let mut map: RelationsMap = HashMap::default();
            map.insert(name, rel.clone());
            HttpResponse::Ok().body(to_string(&gen_graph(&map, category)).unwrap())
        }
        _ => HttpResponse::Ok().body("null"),
    };
    data
}
