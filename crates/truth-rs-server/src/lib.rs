mod graph;
mod relation;
mod tree;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use graph::{graph_json, graph_name_json};
use relation::{relations_json, relations_name_json};
use std::fs;
use tree::tree_json;
use truth_rs_type::RelationsMap;

pub struct AppState {
    relations: RelationsMap,
}

#[get("/")]
async fn index() -> impl Responder {
    let html = fs::read_to_string("dist/index.html").unwrap();
    HttpResponse::Ok().body(html)
}

#[actix_web::main] // or #[tokio::main]
pub async fn start_server(port: u16, relations: RelationsMap) -> std::io::Result<()> {
    HttpServer::new(move || {
        let cors = actix_cors::Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(AppState {
                relations: relations.clone(),
            }))
            .service(
                web::scope("/api")
                    .service(index)
                    .service(relations_json)
                    .service(relations_name_json)
                    .service(graph_json)
                    .service(graph_name_json)
                    .service(tree_json),
            )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
