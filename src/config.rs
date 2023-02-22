use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub path: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub package: Package,
}
