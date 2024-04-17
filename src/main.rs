use axum::{
    routing::{get, post},
    Json, Router,
};
mod modules;
use modules::{orders::Orders, pcenters::PCenters, http_body::HttpBody};
use local_ip_address::local_ip;



async fn list_pcenters() -> Json<Vec<PCenters>>{
    let p_centers: Vec<PCenters> = vec![
        PCenters{
            pk_center: 1,
            p_center_description: String::from("Pizzeria"),
            detination: String::from("something")
        }
    ];

    return Json(p_centers);
}

async fn list_orders(body: String) -> Json<Vec<Orders>> {
    let mut _body: HttpBody = serde_json::from_str(&body).unwrap();
    
    let orders: Vec<Orders> = vec![
        Orders {
            pk_order: 1,
            opening_datetime: None,
            closing_datetime: None,
            waiter: String::from("John Doe"),
            status: 0, //Stato di completamento dell'ordine
            order_destination: String::from("E"),//T: TakeAway, E: Eat-In, D: Delivery
            dish: 0,
            max_dish: 2,
            allarm_status: 0,
            fk_bill: None,
            fk_resource: None,
            covers: 2, 

            //Identificazione del tavolo
            resource_desctipion: String::from("Tavolo 1"), //es. "Tavolo 1"
            resource_signature: String::from("1"), //Tavolo vero e proprio

            bill_details: vec![],
        },
    ];


    return Json(orders)
}

#[tokio::main]
async fn main() {
    let my_local_ip = local_ip().unwrap();
    // Define Routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, Rust!" }))
        .route("/pcenters", get(list_pcenters))
        .route("/orders", post(list_orders));

    println!("Running on http://{:?}:3000", my_local_ip);
    let address = format!("{}:3000", my_local_ip);
    // Start Server
    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
