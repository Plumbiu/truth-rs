use serde::Serialize;

#[derive(Serialize)]
pub struct Pkgs {
    pub name: String,
    pub version: String,
    pub packages: Vec<Pkgs>,
}
