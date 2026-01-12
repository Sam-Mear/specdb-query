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
    // Respect configuration: disallow adding extras when disabled
    if !state.allow_extras {
        return Err(StatusCode::FORBIDDEN);
    }
    // Basic validation
    if payload.spec_name.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    if payload.key.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let extra = match payload.extra {
        Some(e) => e,
        None => return Err(StatusCode::BAD_REQUEST),
    };

    // Insert/update into in-memory extras_map (use write lock)
    let mut outer = state.query_state.extras_map.write().await;
    // get or create per-spec map
    let spec_entry = outer.entry(payload.spec_name.clone()).or_insert_with(HashMap::new);
    // get or create per-section map
    let section_map = spec_entry.entry(payload.section_header.clone()).or_insert_with(HashMap::new);
    section_map.insert(payload.key.clone(), extra.clone());

    // Build response with new version (echoing provided version if present)
    let resp = AddExtraResponse {
        ok: true,
        message: "ok".to_string(),
        version: extra.version,
        extra: Some(extra),
    };

    Ok(Protobuf(resp))
}


#[axum::debug_handler]
pub async fn get_handler(
    State(state): State<Arc<AppState>>,
    Path(spec_name): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Protobuf<GetExtrasResponse>, StatusCode> {
    // read lock
    let outer = state.query_state.extras_map.read().await;
    let mut response = GetExtrasResponse { extras_by_section: HashMap::new() };

    let section_filter = params.get("section_header");
    if let Some(spec_map) = outer.get(&spec_name) {
        if let Some(section) = section_filter {
            if let Some(section_map) = spec_map.get(section) {
                let mut s = SectionExtras { extras: HashMap::new() };
                for (k, v) in section_map.iter() {
                    s.extras.insert(k.clone(), v.clone());
                }
                response.extras_by_section.insert(section.clone(), s);
            }
        } else {
            for (section, section_map_any) in spec_map.iter() {
                let mut s = SectionExtras { extras: HashMap::new() };
                for (k, v) in section_map_any.iter() {
                    s.extras.insert(k.clone(), v.clone());
                }
                response.extras_by_section.insert(section.clone(), s);
            }
        }
    }

    Ok(Protobuf(response))
}