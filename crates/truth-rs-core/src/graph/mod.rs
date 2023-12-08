use crate::util::merge_map;
use ahash::RandomState;
use std::collections::HashSet;
use truth_rs_type::{
    graph::{Graph, GraphLink, GraphNode},
    RelationsMap,
};

pub fn gen_graph(relations: &RelationsMap) -> Graph {
    let mut graph = Graph {
        nodes: Vec::default(),
        links: Vec::default(),
    };
    let mut nodes_set: HashSet<String, RandomState> = HashSet::default();
    for (source, rel) in relations {
        nodes_set.insert(source.to_owned());
        if let Some(deps) = &merge_map(&rel.dependencies, &rel.devDependencies) {
            for target in deps.keys() {
                nodes_set.insert(target.to_owned());
                graph.links.push(GraphLink {
                    source: source.to_owned(),
                    target: target.to_owned(),
                })
            }
        }
    }
    for id in nodes_set {
        graph.nodes.push(GraphNode {
            name: id.clone(),
            value: id,
        })
    }
    graph
}

pub fn stringify_graph(relations: &RelationsMap) -> String {
    serde_json::to_string(&gen_graph(relations)).unwrap()
}
