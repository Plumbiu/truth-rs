use serde::Serialize;

// tree is simlar to json type, but g6 require `id` and `children` field
#[derive(Serialize)]
pub struct Tree {
    pub id: Option<String>,
    pub label: String,
    pub children: Option<Vec<Tree>>,
}
