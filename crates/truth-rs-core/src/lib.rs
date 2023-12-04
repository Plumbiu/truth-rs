pub mod graph;
pub mod json;
mod lock_file;
mod util;

use lock_file::{update_by_npm, update_by_pnpm, update_by_yarn};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use truth_rs_type::{npm::NpmRelation, Relation, RelationsMap};
use util::merge_map;

pub fn gen_relations() -> RelationsMap {
    let mut relations: RelationsMap = HashMap::default();
    let base = fs::read_to_string("package.json").unwrap();
    let NpmRelation {
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
            packages: merge_map(&dependencies, &devDependencies),
        },
    );
    if Path::new("package-lock.json1").exists() {
        update_by_npm(&mut relations);
    } else if Path::new("yarn.lock1").exists() {
        update_by_yarn(&mut relations);
    } else if Path::new("pnpm-lock.yaml").exists() {
        update_by_pnpm(&mut relations);
    }
    relations
}

pub fn write_relation(write_path: &PathBuf) {
    let _ = fs::write(
        write_path.join("relations.json"),
        serde_json::to_string(&gen_relations()).unwrap(),
    );
}
