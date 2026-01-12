use std::sync::Arc;

use axum::{body::Body, extract::State};
use axum_extra::protobuf::Protobuf;

use crate::{AppState, proto_specdb::query::AddExtra};


#[axum::debug_handler]
pub async fn handler(
    Protobuf(payload): Protobuf<AddExtra>
) {
    
}