use serde::{Deserialize, Serialize};
use super::orders_details::OrderDetails;

///Lo statu assume valore: 
/// -1: Quando non è ancora stato evaso l'ordine
/// -2: Quando la portata è parzialmente evasa
/// -3: Quando la portata è completamente evasa
/// 
///order_destination assume valore:
/// -T: Take-Away
/// -E: Eat-In
/// -D: Delivery
#[derive(Deserialize, Serialize)]
pub struct Orders{
    pub pk_order: i32,
    pub opening_datetime: Option<String>,
    pub closing_datetime: Option<String>,
    pub waiter: String,
    pub status: i32, //Stato di completamento dell'ordine
    pub order_destination: String,//T: TakeAway, E: Eat-In, D: Delivery
    pub dish: i32,
    pub max_dish: i32,
    pub allarm_status: i32,
    pub fk_bill: Option<i32>,
    pub fk_resource: Option<i32>,
    pub covers: i32, 

    //Identificazione del tavolo
    pub resource_desctipion: String, //es. "Tavolo 1"
    pub resource_signature: String, //Tavolo vero e proprio

    pub bill_details: Vec<OrderDetails>,
}