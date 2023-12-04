use crate::AHashMap;
use ahash::RandomState;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct PnpmPkgsItem {
    pub dependencies: Option<AHashMap>,
}

#[derive(Deserialize, Debug)]
pub struct PnpmLock {
    pub packages: Option<HashMap<String, PnpmPkgsItem, RandomState>>,
}
