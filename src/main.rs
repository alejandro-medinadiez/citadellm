
//region --- Imports -----------------------------------------

use axum::{routing::get, Router};

//endregion --------------------------------------------------


//region --- Main  -------------------------------------------


#[tokio::main]
async fn main(){
    let app = Router::new().route("/health", get(|| async {"OK\n"}));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


//endregion --------------------------------------------------