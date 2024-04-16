use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct OrderDetails{
    pub pk_order_detail: i32,
    pub product_description: String,
    pub qty: i32,
    pub fk_product: i32,
    pub fk_pcenter: i32,
    pub pcenter_description: String,
    pub qty_prepared: i32,

    //Campi per le richiede di modifica
    pub update_requested: i32,
    pub update_acknowledged: i32,
    pub update_reason: Option<String>,

    pub preparation_ended: String,
    pub variations: Vec<String>



}