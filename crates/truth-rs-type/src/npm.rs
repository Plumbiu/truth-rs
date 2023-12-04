use ahash::RandomState;
use serde::Deserialize;
use std::collections::HashMap;

use crate::AHashMap;

/// package field in package-lock.json
#[derive(Deserialize, Debug)]
pub struct NpmRelation {
    pub name: Option<String>,
    pub version: String,
    pub devDependencies: Option<AHashMap>,
    pub dependencies: Option<AHashMap>,
}

pub type NpmLockPackages = HashMap<String, NpmRelation, RandomState>;

/// for deserialize package-lock.json field
#[derive(Deserialize, Debug)]
pub struct NpmLock {
    pub name: Option<String>,
    pub version: Option<String>,
    pub lockfileVersion: u8,
    pub requires: bool,
    pub packages: NpmLockPackages,
}
