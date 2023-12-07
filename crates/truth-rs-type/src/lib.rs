#![allow(non_snake_case)]
pub mod graph;
pub mod json;
pub mod package;
pub mod tree;

use ahash::RandomState;
use serde::{Deserialize, Serialize};
use std::{collections::{HashMap, HashSet}, ffi::OsString};

pub type AHashMap = HashMap<String, String, RandomState>;
pub type AHashSet = HashSet<String, RandomState>;
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Relation {
    pub name: String,
    pub version: String,
    pub path: OsString,
    pub dependencies: Option<AHashMap>,
    pub devDependencies: Option<AHashMap>,
    pub homepage: Option<String>,
}
pub type RelationsMap = HashMap<String, Relation, RandomState>;
