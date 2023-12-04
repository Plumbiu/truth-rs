use crate::util::merge_map;
use regex::Regex;
use std::{collections::HashMap, fs};
use truth_rs_type::{npm::NpmLock, pnpm::PnpmLock, AHashMap, Relation, RelationsMap};

pub fn update_by_yarn(relations: &mut RelationsMap) {
    let lock_file = fs::read_to_string("yarn.lock").unwrap();
    let lock_file = yarn_lock_parser::parse_str(&lock_file).unwrap();
    for lock_item in lock_file {
        let mut deps: AHashMap = HashMap::default();
        for (name, version) in lock_item.dependencies {
            deps.insert(name.to_owned(), version.to_owned());
        }
        let rel = Relation {
            name: Some(lock_item.name.to_owned()),
            version: lock_item.version.to_owned(),
            packages: Some(deps),
        };

        relations.insert(lock_item.name.to_owned(), rel);
    }
}

pub fn update_by_pnpm(relations: &mut RelationsMap) {
    let re = Regex::new("@\\d+.\\d+.\\d+").unwrap();
    let lock_file = fs::read_to_string("pnpm-lock.yaml").unwrap();
    let lock_file: PnpmLock = serde_yaml::from_str(&lock_file).unwrap();
    if let Some(pkgs) = lock_file.packages {
        for (key, item) in pkgs {
            let matcher = re.find(key.as_str()).unwrap();
            let name = key[1..matcher.start()].to_owned();
            relations.insert(
                name.clone(),
                Relation {
                    name: Some(name),
                    version: key[matcher.start() + 1..matcher.end()].to_owned(),
                    packages: item.dependencies,
                },
            );
        }
    }
}

pub fn update_by_npm(relations: &mut RelationsMap) {
    let lock_file = fs::read_to_string("package-lock.json").unwrap();
    let lock_file: NpmLock = serde_json::from_str(&lock_file).unwrap();
    const NODE_MODULES: &str = "node_modules/";
    const NODE_MODULES_LEN: usize = NODE_MODULES.len();
    for (mut name, rel) in lock_file.packages {
        while let Some(idx) = name.find(NODE_MODULES) {
            name = name[idx + NODE_MODULES_LEN..].to_owned();
        }
        relations.insert(
            name.clone(),
            Relation {
                name: Some(name),
                version: rel.version,
                packages: merge_map(&rel.dependencies, &rel.devDependencies),
            },
        );
    }
}
