use serde::Serialize;

// tree is simlar to json type, but g6 require `id` and `children` field
#[derive(Serialize)]
pub struct Tree {
    pub id: String,
    pub name: String,
    pub version: String,
    pub children: Vec<Tree>,
}
