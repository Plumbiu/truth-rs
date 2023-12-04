use std::{fs, path::PathBuf, vec};
use truth_rs_type::{tree::Tree, AHashMap, RelationsMap};

use crate::gen_relations;

fn map2vec(v: &mut Vec<Tree>, m: &Option<AHashMap>) {
    if let Some(map) = m {
        for (id, label) in map.iter() {
            v.push(Tree {
                id: Some(id.to_owned()),
                label: label.to_owned(),
                children: Some(vec![]),
            })
        }
    }
}

fn do_gen_tree(root: &mut Vec<Tree>, max_dep: u16, relation_map: &RelationsMap) {
    if max_dep <= 0 {
        return;
    }
    for item in root {
        if let Some(id) = &item.id {
            println!("{id}");
            let tree = relation_map.get(id);
            if let Some(rel) = tree {
                if let Some(packages) = &mut item.children {
                    map2vec(packages, &rel.packages);
                    do_gen_tree(packages, max_dep - 1, relation_map)
                }
            }
        }
    }
}

pub fn gen_tree(depth: u16, relation_map: &RelationsMap) -> Tree {
    let mut root_pkg = match relation_map.get("__root__") {
        Some(rel) => {
            println!("{:?}", rel);
            let mut tmp: Vec<Tree> = Vec::new();
            map2vec(&mut tmp, &rel.packages);
            Tree {
                id: Some("__root__".to_owned()),
                label: rel.version.to_owned(),
                children: Some(tmp),
            }
        }
        _ => panic!(),
    };

    if let Some(tree) = &mut root_pkg.children {
        do_gen_tree(tree, depth, &relation_map)
    }
    root_pkg
}

pub fn write_tree(depth: u16, write_path: &PathBuf) {
    let _ = fs::write(
        write_path.join("tree.json"),
        serde_json::to_string(&gen_tree(depth, &gen_relations())).unwrap(),
    );
}
