use std::sync::Arc;
use axum::{extract::State, http::StatusCode};
use axum_extra::protobuf::Protobuf;
use std::collections::HashMap;

use crate::{AppState, proto_specdb::{self, query::{AddExtraRequest, AddExtraResponse}}};
use axum::{extract::{Path, Query}};
use proto_specdb::query::{GetExtrasResponse, SectionExtras};

#[axum::debug_handler]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Protobuf(payload): Protobuf<AddExtraRequest>
) -> Result<Protobuf<AddExtraResponse>, StatusCode> {
    if !state.allow_extras { return Err(StatusCode::FORBIDDEN); }
    if payload.spec_name.is_empty() { return Err(StatusCode::BAD_REQUEST); }
    if payload.key.is_empty() { return Err(StatusCode::BAD_REQUEST); }
    let extra = match payload.extra { Some(e) => e, None => return Err(StatusCode::BAD_REQUEST) };

    let mut outer = state.query_state.extras_map.write().await;
    let spec_entry = outer.entry(payload.spec_name.clone()).or_insert_with(HashMap::new);
    let section_map = spec_entry.entry(payload.section_header.clone()).or_insert_with(HashMap::new);
    section_map.insert(payload.key.clone(), extra.clone());

    let resp = AddExtraResponse { ok: true, message: "ok".to_string(), version: extra.version, extra: Some(extra) };
    Ok(Protobuf(resp))
}


#[axum::debug_handler]
pub async fn get_handler(
    State(state): State<Arc<AppState>>,
    Path(spec_name): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Protobuf<GetExtrasResponse>, StatusCode> {
    let outer = state.query_state.extras_map.read().await;
    let mut response = GetExtrasResponse { extras_by_section: HashMap::new() };
    let section_filter = params.get("section_header");
    if let Some(spec_map) = outer.get(&spec_name) {
        if let Some(section) = section_filter {
            if let Some(section_map) = spec_map.get(section) {
                let mut s = SectionExtras { extras: HashMap::new() };
                for (k, v) in section_map.iter() { s.extras.insert(k.clone(), v.clone()); }
                response.extras_by_section.insert(section.clone(), s);
            }
        } else {
            for (section, section_map_any) in spec_map.iter() {
                let mut s = SectionExtras { extras: HashMap::new() };
                for (k, v) in section_map_any.iter() { s.extras.insert(k.clone(), v.clone()); }
                response.extras_by_section.insert(section.clone(), s);
            }
        }
    }
    Ok(Protobuf(response))
}


#[axum::debug_handler]
pub async fn export_handler(
    State(state): State<Arc<AppState>>,
    Path(spec_name): Path<String>,
) -> Result<Protobuf<GetExtrasResponse>, StatusCode> {
    let outer = state.query_state.extras_map.read().await;
    let mut response = GetExtrasResponse { extras_by_section: HashMap::new() };
    if let Some(spec_map) = outer.get(&spec_name) {
        for (section, section_map_any) in spec_map.iter() {
            let mut s = SectionExtras { extras: HashMap::new() };
            for (k, v) in section_map_any.iter() { s.extras.insert(k.clone(), v.clone()); }
            response.extras_by_section.insert(section.clone(), s);
        }
    }
    Ok(Protobuf(response))
}


#[axum::debug_handler]
pub async fn import_handler(
    State(state): State<Arc<AppState>>,
    Path(spec_name): Path<String>,
    Protobuf(payload): Protobuf<GetExtrasResponse>,
) -> Result<StatusCode, StatusCode> {
    if !state.allow_extras { return Err(StatusCode::FORBIDDEN); }
    let mut outer = state.query_state.extras_map.write().await;
    let spec_entry = outer.entry(spec_name.clone()).or_insert_with(HashMap::new);
    for (section, section_extras) in payload.extras_by_section.into_iter() {
        let section_map = spec_entry.entry(section.clone()).or_insert_with(HashMap::new);
        for (k, v) in section_extras.extras.into_iter() { section_map.insert(k, v); }
    }
    Ok(StatusCode::OK)
}
