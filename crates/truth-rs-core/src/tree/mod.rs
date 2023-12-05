use crate::gen_relations;
use std::{collections::HashSet, fs, path::PathBuf, vec};
use truth_rs_type::{tree::Tree, AHashMap, RelationsMap};

fn map2vec(v: &mut Vec<Tree>, m: &Option<AHashMap>, set: &mut HashSet<String, ahash::RandomState>) {
    if let Some(map) = m {
        for (id, label) in map.iter() {
            let mut uid = id.to_owned();
            while let Some(_id) = set.get(&uid) {
                uid = format!("{}_", uid);
            }
            v.push(Tree {
                id: Some(uid.to_owned()),
                label: label.to_owned(),
                children: Some(vec![]),
            });
            set.insert(uid.to_owned());
        }
    }
}

fn do_gen_tree(
    root: &mut Vec<Tree>,
    max_dep: u16,
    relation_map: &RelationsMap,
    set: &mut HashSet<String, ahash::RandomState>,
) {
    if max_dep <= 0 {
        return;
    }
    let mut tree_set: HashSet<String, ahash::RandomState> = HashSet::default();
    for item in root {
        if let Some(id) = &item.id {
            let tree = relation_map.get(id);
            if let Some(rel) = tree {
                if let None = tree_set.get(&rel.name) {
                    tree_set.insert(id.to_owned());
                    if let Some(packages) = &mut item.children {
                        map2vec(packages, &rel.packages, set);
                        do_gen_tree(packages, max_dep - 1, relation_map, set);
                        tree_set.remove(id);
                    }
                }
            }
        }
    }
}

pub fn gen_tree(depth: u16, relation_map: &RelationsMap) -> Tree {
    let mut set: HashSet<String, ahash::RandomState> = HashSet::default();
    let mut root_pkg = match relation_map.get("__root__") {
        Some(rel) => {
            let mut tmp: Vec<Tree> = Vec::new();
            map2vec(&mut tmp, &rel.packages, &mut set);
            Tree {
                id: Some("__root__".to_owned()),
                label: rel.version.to_owned(),
                children: Some(tmp),
            }
        }
        _ => panic!(),
    };

    if let Some(tree) = &mut root_pkg.children {
        do_gen_tree(tree, depth, &relation_map, &mut set);
    }
    root_pkg
}

pub fn write_tree(depth: u16, write_path: &PathBuf) {
    let _ = fs::write(
        write_path.join("tree.json"),
        serde_json::to_string(&gen_tree(depth, &gen_relations())).unwrap(),
    );
}
