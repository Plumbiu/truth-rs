use serde::Serialize;

#[derive(Serialize)]
pub struct GraphNode {
    pub name: String,
    pub value: String,
}

#[derive(Serialize)]
pub struct GraphLink {
    pub source: String,
    pub target: String,
}

#[derive(Serialize)]
pub struct Graph {
    pub nodes: Vec<GraphNode>,
    pub links: Vec<GraphLink>,
}
