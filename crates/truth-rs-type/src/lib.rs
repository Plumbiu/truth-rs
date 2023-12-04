#![allow(non_snake_case)]
pub mod graph;
pub mod json;
pub mod npm;
pub mod pnpm;
pub mod tree;

use ahash::RandomState;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type AHashMap = HashMap<String, String, RandomState>;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Relation {
    pub name: Option<String>,
    pub version: String,
    pub packages: Option<AHashMap>,
}
pub type RelationsMap = HashMap<String, Relation, RandomState>;
