use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct HelloParams {
    pub name: Option<String>
}