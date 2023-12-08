use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use truth_rs_type::{package::Package, Relation, RelationsMap};

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
            path: dir.as_os_str().to_os_string(),
            version: pkg.version,
            dependencies: pkg.dependencies,
            devDependencies: pkg.devDependencies,
            homepage: pkg.homepage,
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
        let dir = dir.unwrap();
        if let Some(name) = dir.file_name().to_str() {
            if name.starts_with(".") {
                continue;
            } else if name.starts_with("@") {
                insert_relations_many(relations, &dir.path(), None);
            } else {
                insert_relations_one(relations, &dir.path(), None);
            }
        }
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

pub fn stringify_relations() -> String {
    serde_json::to_string(&gen_relations()).unwrap()
}
