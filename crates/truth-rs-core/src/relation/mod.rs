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
            devDependencies: match include_dev {
                true => pkg.devDependencies,
                _ => None,
            },
            homepage: pkg.homepage,
        },
    );
}

fn insert_relations_many(
    relations: &mut RelationsMap,
    dir: &PathBuf,
    name: Option<String>,
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
                insert_relations_many(relations, &dir.path(), None, include_dev);
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
        include_dev,
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
    serde_json::to_string(&gen_relations(include_dev)).unwrap()
}
