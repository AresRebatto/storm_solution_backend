use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct HttpBody{
    pub p_center: String,
    pub date_time_from: String,
    pub date_time_to: String,
    pub order_type: i32,
    pub pk_order: i32,
}