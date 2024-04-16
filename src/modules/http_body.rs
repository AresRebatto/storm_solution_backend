use serde_json::Value;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct HttpBody{
    pub gatto: Value,
}