use crate::util::merge_map;
use std::collections::HashSet;
use truth_rs_type::{tree::Tree, AHashMap, RelationsMap};

fn insert_tree(
    v: &mut Vec<Tree>,
    m: &Option<AHashMap>,
    set: &mut HashSet<String, ahash::RandomState>,
) {
    if let Some(map) = m {
        for (id, version) in map {
            let mut uid = id.to_owned();
            while let Some(_id) = set.get(&uid) {
                uid = uid + "_";
            }
            v.push(Tree {
                id: uid.to_owned(),
                name: id.to_owned(),
                version: version.to_owned(),
                children: Vec::default(),
            });
            set.insert(uid.to_owned());
        }
    }
}

fn do_gen_tree(
    root: &mut Vec<Tree>,
    max_dep: u8,
    relation_map: &RelationsMap,
    set: &mut HashSet<String, ahash::RandomState>,
) {
    if max_dep <= 0 {
        return;
    }
    let mut tree_set: HashSet<String, ahash::RandomState> = HashSet::default();
    for item in root {
        let id = &item.id;
        let tree = relation_map.get(id);
        if let Some(rel) = tree {
            if let None = tree_set.get(&rel.name) {
                let packages = &mut item.children;
                tree_set.insert(id.to_owned());
                insert_tree(
                    packages,
                    &merge_map(&rel.dependencies, &rel.devDependencies),
                    set,
                );
                do_gen_tree(packages, max_dep - 1, relation_map, set);
                tree_set.remove(id);
            }
        }
    }
}

pub fn gen_tree(depth: u8, relation_map: &RelationsMap) -> Tree {
    let mut set: HashSet<String, ahash::RandomState> = HashSet::default();
    let mut root_pkg = match relation_map.get("__root__") {
        Some(rel) => {
            let mut tmp: Vec<Tree> = Vec::new();
            insert_tree(
                &mut tmp,
                &merge_map(&rel.dependencies, &rel.devDependencies),
                &mut set,
            );
            Tree {
                id: "__root__".to_owned(),
                name: rel.name.to_owned(),
                version: rel.version.to_owned(),
                children: tmp,
            }
        }
        _ => panic!(),
    };

    do_gen_tree(&mut root_pkg.children, depth, &relation_map, &mut set);
    root_pkg
}

pub fn stringify_tree(relations: &RelationsMap, depth: u8) -> String {
    simd_json::to_string(&gen_tree(depth, relations)).unwrap()
}
