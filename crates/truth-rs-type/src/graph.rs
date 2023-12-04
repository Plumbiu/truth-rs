use serde::Serialize;

#[derive(Serialize)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
}

#[derive(Serialize)]
pub struct GraphLink {
    pub source: String,
    pub target: String,
    pub value: String,
}

#[derive(Serialize)]
pub struct Graph {
    pub nodes: Vec<GraphNode>,
    pub links: Vec<GraphLink>,
}
