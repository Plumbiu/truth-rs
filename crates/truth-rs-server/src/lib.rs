use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde_json::to_string;
use std::fs;
use truth_rs_core::{graph::gen_graph, tree::stringify_tree};
use truth_rs_type::RelationsMap;
struct AppState {
    relations: RelationsMap,
}

#[get("/graph.json")]
async fn graph_json(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(to_string(&gen_graph(&data.relations)).unwrap())
}

#[get("/relations.json")]
async fn relations_json(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(to_string(&data.relations).unwrap())
}

#[get("/tree.json")]
async fn tree_json(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(stringify_tree(&data.relations, 3))
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
                    .service(graph_json)
                    .service(tree_json),
            )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
