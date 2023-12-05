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

pub fn do_gen_relations(relations: &mut RelationsMap, p: &Path) {
    let dirs = p.read_dir().unwrap();
    for dir in dirs {
        let entry = dir.unwrap();
        let path = entry.path();
        let pkg_path = path.join("package.json");
        if pkg_path.exists() {
            let pkg = fs::read_to_string(pkg_path).unwrap();
            let pkg: Package = serde_json::from_str(&pkg).unwrap();
            relations.insert(
                pkg.name.to_owned(),
                Relation {
                    name: pkg.name,
                    version: pkg.version,
                    packages: merge_map(pkg.dependencies, pkg.devDependencies),
                },
            );
        } else if path.is_dir() {
            do_gen_relations(relations, &path);
        }
    }
}

pub fn gen_relations() -> RelationsMap {
    let mut relations: RelationsMap = HashMap::default();
    let base = fs::read_to_string("package.json").unwrap();
    let Package {
        name,
        version,
        dependencies,
        devDependencies,
    } = serde_json::from_str(&base).unwrap();
    relations.insert(
        "__root__".to_owned(),
        Relation {
            name,
            version,
            packages: merge_map(dependencies, devDependencies),
        },
    );
    do_gen_relations(&mut relations, Path::new("node_modules"));
    relations
}

pub fn write_relation(write_path: &PathBuf) {
    let _ = fs::write(
        write_path.join("relations.json"),
        serde_json::to_string(&gen_relations()).unwrap(),
    );
}
