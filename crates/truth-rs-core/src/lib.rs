pub mod graph;
pub mod json;
pub mod tree;
mod util;

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use truth_rs_type::{package::Package, Relation, RelationsMap};
use util::merge_map;

fn insert_relations_one(relations: &mut RelationsMap, dir: &PathBuf, name: Option<String>) {
    let package_json_path = Path::join(dir, "package.json");
    let pkg = fs::read(package_json_path).unwrap();
    let pkg: Package = serde_json::from_slice(&pkg).unwrap();
    relations.insert(
        match name {
            Some(t) => t,
            None => pkg.name.to_owned(),
        },
        Relation {
            name: pkg.name,
            version: pkg.version,
            packages: merge_map(pkg.dependencies, pkg.devDependencies),
        },
    );
}

fn insert_relations_many(relations: &mut RelationsMap, dir: &PathBuf, name: Option<String>) {
    let dirs = fs::read_dir(&dir).unwrap();
    for sub_dir in dirs {
        insert_relations_one(relations, &sub_dir.unwrap().path(), name.clone());
    }
}

fn insert_relations(relations: &mut RelationsMap, p: &Path) {
    let dirs = p.read_dir().unwrap();
    for dir in dirs {
        // let path = dir.unwrap().path();
        let dir = dir.unwrap();
        let name = dir.file_name().to_str().unwrap().to_string();
        if name.starts_with(".") {
            if name != String::from(".pnpm") {
                continue;
            }
        } else if name.starts_with("@") {
            insert_relations_many(relations, &dir.path(), None);
        } else {
            insert_relations_one(relations, &dir.path(), None);
        }
        // println!("{:?}", dir.unwrap().file_name());
    }
}

pub fn gen_relations() -> RelationsMap {
    let mut relations: RelationsMap = HashMap::default();
    insert_relations_one(
        &mut relations,
        &PathBuf::from("."),
        Some(String::from("__root__")),
    );
    insert_relations(&mut relations, Path::new("node_modules"));
    if Path::exists(&Path::new("pnpm-lock.yaml")) {
        insert_relations(&mut relations, Path::new("node_modules/.pnpm/node_modules"));
    }
    relations
}

pub fn write_relation(write_path: &PathBuf) {
    let _ = fs::write(
        write_path.join("relations.json"),
        serde_json::to_string(&gen_relations()).unwrap(),
    );
}
