use std::collections::HashMap;
use truth_rs_type::AHashMap;

pub fn merge_map(m1: &Option<AHashMap>, m2: &Option<AHashMap>) -> Option<AHashMap> {
    let mut pkgs: AHashMap = HashMap::default();
    if let Some(deps) = m1 {
        for (name, version) in deps.iter() {
            pkgs.insert(name.to_owned(), version.to_owned());
        }
    }
    if let Some(deps) = m2 {
        for (name, version) in deps.iter() {
            pkgs.insert(name.to_owned(), version.to_owned());
        }
    }
    Some(pkgs)
}
