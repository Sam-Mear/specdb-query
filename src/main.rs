use std::sync::Arc;

use axum::{
    extract::State, routing::get, Json, Router
};
use serde::{Serialize};
use specdb::{get_spec_db, SpecDb};
use tower_http::trace::TraceLayer;

struct AppState {
    spec_db: SpecDb
}

async fn handler(
    State(state): State<Arc<AppState>>
) -> Json<SpecDb>
{
    return Json(state.spec_db.clone());
}

#[tokio::main]
async fn main() {
    let spec_db = get_spec_db("/home/smear/personal/SpecDB/specs".to_string());
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();
    let shared_state = Arc::new(AppState { spec_db });
    
    // println!("Files parsed total: {}", spec_db.files.iter().count());

    // build our application with a single route
    let app = Router::new().route("/", get(handler).with_state(shared_state))
    .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 8082
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8082").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}