use ahash::RandomState;
use std::{collections::HashSet, fs, path::PathBuf, vec};
use truth_rs_type::{
    graph::{Graph, GraphLink, GraphNode},
    RelationsMap,
};

use crate::gen_relations;

pub fn gen_graph(relations: &RelationsMap) -> Graph {
    let mut graph = Graph {
        nodes: vec![],
        links: vec![],
    };
    let mut nodes_set: HashSet<String, RandomState> = HashSet::default();
    for (source, rel) in relations {
        nodes_set.insert(source.to_string());
        if let Some(deps) = &rel.packages {
            for target in deps.keys() {
                nodes_set.insert(target.to_string());
                graph.links.push(GraphLink {
                    source: source.to_string(),
                    target: target.to_string(),
                    value: source.to_string(),
                })
            }
        }
    }
    for id in nodes_set {
        graph.nodes.push(GraphNode {
            id: id.clone(),
            label: id,
        })
    }
    graph
}

pub fn write_graph(write_path: &PathBuf) {
    let _ = fs::write(
        write_path.join("graph.json"),
        serde_json::to_string(&gen_graph(&gen_relations())).unwrap(),
    );
}
