use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PCenters{
    pub pk_center: i32,
    pub p_center_description: String,
    pub detination: String
}