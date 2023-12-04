use std::{fs, vec};
use truth_rs_type::{json::Pkgs, AHashMap, RelationsMap};

use crate::gen_relations;

fn map2vec(v: &mut Vec<Pkgs>, m: &Option<AHashMap>) {
    if let Some(map) = m {
        for (name, rel) in map.iter() {
            v.push(Pkgs {
                name: Some(name.to_owned()),
                version: rel.to_owned(),
                packages: Some(vec![]),
            })
        }
    }
}

fn do_gen_json(root: &mut Vec<Pkgs>, max_dep: u16, relation_map: &RelationsMap) {
    if max_dep <= 0 || root.len() == 0 {
        return;
    }
    for item in root {
        if let Some(name) = &item.name {
            let pkgs = relation_map.get(name);
            if let Some(rel) = pkgs {
                if let Some(packages) = &mut item.packages {
                    map2vec(packages, &rel.packages);
                    do_gen_json(packages, max_dep - 1, relation_map)
                }
            }
        }
    }
}

pub fn gen_json(depth: u16, relation_map: &RelationsMap) -> Pkgs {
    let mut root_pkg = match relation_map.get("__root__") {
        Some(rel) => {
            let mut tmp: Vec<Pkgs> = Vec::new();
            map2vec(&mut tmp, &rel.packages);
            Pkgs {
                name: Some("__root__".to_owned()),
                version: rel.version.to_owned(),
                packages: Some(tmp),
            }
        }
        _ => panic!(),
    };

    if let Some(pkgs) = &mut root_pkg.packages {
        do_gen_json(pkgs, depth, &relation_map)
    }
    root_pkg
}

pub fn write_json(depth: u16, write_path: &str) {
    let _ = fs::write(
        write_path,
        serde_json::to_string(&gen_json(depth, &gen_relations())).unwrap(),
    );
}
