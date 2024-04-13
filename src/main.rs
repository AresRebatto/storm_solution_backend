use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
mod modules;
use modules::orders::Orders;



// Handler for /create-user
async fn create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created successfully"))
        .unwrap()
}
// Handler for /users
async fn list_orders() -> Json<Vec<Orders>> {
    let orders = vec![
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
    Json(orders)
}

#[tokio::main]
async fn main() {
    // Define Routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, Rust!" }))
        .route("/create-user", post(create_user))
        .route("/orders", get(list_orders));

    println!("Running on http://localhost:3000");
    // Start Server
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
