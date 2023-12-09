use serde::Deserialize;

#[derive(Deserialize)]
pub struct GraphQuery {
    pub name: String,
    pub category: Option<u8>,
}
