use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct BaseBody {
    pub r#type: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BaseData {
    pub src: String,
    pub dest: String,
    pub body: BaseBody,
}
