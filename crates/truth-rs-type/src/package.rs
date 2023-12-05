use serde::{Deserialize, Serialize};
use crate::AHashMap;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub dependencies: Option<AHashMap>,
    pub devDependencies: Option<AHashMap>,
}
