use serde::Serialize;

#[derive(Serialize)]
pub struct Pkgs {
    pub name: Option<String>,
    pub version: String,
    pub packages: Option<Vec<Pkgs>>,
}
