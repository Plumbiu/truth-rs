use crate::util::merge_map;
use std::collections::HashMap;
use truth_rs_type::{
    graph::{Graph, GraphLink, GraphNode},
    AHashMap, RelationsMap,
};

pub fn gen_graph(relations: &RelationsMap, category: Option<u8>) -> Graph {
    let mut graph = Graph {
        nodes: Vec::default(),
        links: Vec::default(),
    };
    let mut nodes_set: AHashMap = HashMap::default();
    for (source, rel) in relations {
        nodes_set.insert(source.to_owned(), rel.version.to_owned());
        if let Some(deps) = &merge_map(&rel.dependencies, &rel.devDependencies) {
            for (target, version) in deps {
                nodes_set.insert(target.to_owned(), version.to_owned());
                graph.links.push(GraphLink {
                    source: source.to_owned(),
                    target: target.to_owned(),
                })
            }
        }
    }
    for (name, value) in nodes_set {
        graph.nodes.push(GraphNode {
            name,
            value,
            category: match category {
                Some(c) => c,
                _ => 0,
            },
        })
    }
    graph
}

pub fn stringify_graph(relations: &RelationsMap) -> String {
    serde_json::to_string(&gen_graph(relations, None)).unwrap()
}
