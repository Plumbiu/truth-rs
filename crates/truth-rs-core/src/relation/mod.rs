use simd_json;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use truth_rs_type::{package::Package, Relation, RelationsMap};

fn insert_relations_one(
    relations: &mut RelationsMap,
    dir: &PathBuf,
    name: Option<String>,
    include_dev: bool,
) {
    let package_json_path = dir.join("package.json");
    let mut pkg = fs::read_to_string(package_json_path.clone()).unwrap();
    let pkg: Package = unsafe { simd_json::serde::from_str(&mut pkg).unwrap() };
    let mut relation = Relation {
        name: pkg.name.to_owned(),
        path: package_json_path.to_string_lossy().to_string(),
        version: pkg.version,
        dependencies: pkg.dependencies,
        devDependencies: None,
        homepage: pkg.homepage,
    };

    match name {
        Some(t) => {
            relation.devDependencies = pkg.devDependencies;
            relations.insert(t, relation);
        }
        _ => {
            if include_dev == true {
                relation.dependencies = pkg.devDependencies;
            }
            relations.insert(pkg.name, relation);
        }
    };
}

fn insert_relations_many(
    relations: &mut RelationsMap,
    dir: &PathBuf,
    name: &Option<String>,
    include_dev: bool,
) {
    let dirs = fs::read_dir(&dir).unwrap();
    for sub_dir in dirs {
        insert_relations_one(
            relations,
            &sub_dir.unwrap().path(),
            name.clone(),
            include_dev,
        );
    }
}

fn insert_relations(relations: &mut RelationsMap, p: &Path, include_dev: bool) {
    let dirs = p.read_dir().unwrap();
    for dir in dirs {
        let dir = dir.unwrap();
        if let Some(name) = dir.file_name().to_str() {
            if name.starts_with(".") {
                continue;
            } else if name.starts_with("@") {
                insert_relations_many(relations, &dir.path(), &None, include_dev);
            } else {
                insert_relations_one(relations, &dir.path(), None, include_dev);
            }
        }
    }
}

pub fn gen_relations(include_dev: bool) -> RelationsMap {
    let mut relations: RelationsMap = HashMap::default();
    insert_relations_one(
        &mut relations,
        &PathBuf::from("."),
        Some(String::from("__root__")),
        true,
    );
    insert_relations(&mut relations, Path::new("node_modules"), include_dev);
    if Path::exists(&Path::new("pnpm-lock.yaml")) {
        insert_relations(
            &mut relations,
            Path::new("node_modules/.pnpm/node_modules"),
            include_dev,
        );
    }
    relations
}

pub fn stringify_relations(include_dev: bool) -> String {
    simd_json::to_string(&gen_relations(include_dev)).unwrap()
}
