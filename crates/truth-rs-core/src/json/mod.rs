use crate::util::merge_map;
use std::collections::HashSet;
use truth_rs_type::{json::Pkgs, AHashMap, AHashSet, RelationsMap};

fn insert_json(v: &mut Vec<Pkgs>, m: &Option<AHashMap>) {
    if let Some(map) = m {
        for (name, version) in map.iter() {
            v.push(Pkgs {
                name: name.to_owned(),
                version: version.to_owned(),
                packages: Vec::default(),
            })
        }
    }
}

fn do_gen_json(
    root: &mut Vec<Pkgs>,
    max_dep: u8,
    relation_map: &RelationsMap,
    tree_set: &mut AHashSet,
    deep_optimize: bool,
) {
    if max_dep <= 0 {
        return;
    }
    for item in root {
        let name = &item.name;
        let pkgs = relation_map.get(name);
        if let Some(rel) = pkgs {
            let packages = &mut item.packages;
            if let None = tree_set.get(name) {
                tree_set.insert(name.to_owned());
                insert_json(
                    packages,
                    &merge_map(&rel.dependencies, &rel.devDependencies),
                );
                do_gen_json(packages, max_dep - 1, relation_map, tree_set, deep_optimize);
                let _ = deep_optimize || tree_set.remove(&name.to_owned());
            }
        }
    }
}

pub fn gen_json(depth: u8, relation_map: &RelationsMap) -> Pkgs {
    let mut tree_set: AHashSet = HashSet::default();
    let mut root_pkg = match relation_map.get("__root__") {
        Some(rel) => {
            let mut tmp: Vec<Pkgs> = Vec::new();
            insert_json(
                &mut tmp,
                &merge_map(&rel.dependencies, &rel.devDependencies),
            );
            Pkgs {
                name: "__root__".to_owned(),
                version: rel.version.to_owned(),
                packages: tmp,
            }
        }
        _ => panic!(),
    };

    do_gen_json(
        &mut root_pkg.packages,
        depth,
        &relation_map,
        &mut tree_set,
        depth > 5,
    );
    root_pkg
}

pub fn stringify_json(relations: &RelationsMap, depth: u8) -> String {
    sonic_rs::to_string(&gen_json(depth, relations)).unwrap()
}
