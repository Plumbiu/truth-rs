use std::{collections::HashSet, fs, path::PathBuf, vec};
use truth_rs_type::{json::Pkgs, AHashMap, AHashSet, RelationsMap};

use crate::{gen_relations, util::merge_map};

fn map2vec(v: &mut Vec<Pkgs>, m: &Option<AHashMap>) {
    if let Some(map) = m {
        for (name, version) in map.iter() {
            v.push(Pkgs {
                name: name.to_owned(),
                version: version.to_owned(),
                packages: vec![],
            })
        }
    }
}

fn do_gen_json(
    root: &mut Vec<Pkgs>,
    max_dep: u16,
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
                map2vec(
                    packages,
                    &merge_map(&rel.dependencies, &rel.devDependencies),
                );
                do_gen_json(packages, max_dep - 1, relation_map, tree_set, deep_optimize);
                let _ = deep_optimize || tree_set.remove(&name.to_owned());
            }
        }
    }
}

pub fn gen_json(depth: u16, relation_map: &RelationsMap) -> Pkgs {
    let mut tree_set: AHashSet = HashSet::default();
    let mut root_pkg = match relation_map.get("__root__") {
        Some(rel) => {
            let mut tmp: Vec<Pkgs> = Vec::new();
            map2vec(
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

pub fn write_json(depth: u16, write_path: &PathBuf) {
    let _ = fs::write(
        write_path,
        serde_json::to_string(&gen_json(depth, &gen_relations())).unwrap(),
    );
}
