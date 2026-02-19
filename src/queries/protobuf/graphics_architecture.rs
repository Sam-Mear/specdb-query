use crate::{AppState, proto_specdb::{self, query::Extra}};
use std::{collections::HashMap, sync::Arc};

use axum::{extract::{Path, State}, http::StatusCode};
use rapidhash::{HashMapExt, RapidHashMap};
use specdb::{SpecDb, spectype::Type};
use axum_extra::protobuf::Protobuf;

pub fn get_state(specdb: &SpecDb) -> RapidHashMap<String, proto_specdb::query::GraphicsArchitecture>
{
    let mut map = RapidHashMap::<String, proto_specdb::query::GraphicsArchitecture>::new();
    for spec in &specdb.files {
        let proto_spec = match &spec.part_type {
            Type::Cpu(_cpu) => None,
            Type::Apu(_apu) => None,
            Type::GraphicsCard(_graphics_card) => None,
            Type::CpuArchitecture(_cpu_architecture) => None,
            Type::ApuArchitecture(_apu_architecture) => None,
            Type::GraphicsArchitecture(graphics_architecture) => Some(proto_specdb::query::GraphicsArchitecture {
                sections: graphics_architecture.sections.iter().map(
                    |section| proto_specdb::query::Section{
                        header: section.header.clone(),
                        members: section.members.clone(),
                        extras: HashMap::<String, Extra>::new(),
                    }).collect(),
                lithography: graphics_architecture.lithography.0.clone(),
                release_date: graphics_architecture.release_date.0.clone(),
                direct_x_support: match graphics_architecture.direct_x_support.clone() { Some(value) => Some(value.0), None => None},
                vulkan_support: match graphics_architecture.vulkan_support.clone() { Some(value) => Some(value.0), None => None},
                manufacturer: match graphics_architecture.manufacturer.clone() { Some(value) => Some(value.0), None => None},
            }),
            Type::GenericContainer(_generic_container) => None,
            Type::Hidden(_inherit_data) => None,
        };
        match proto_spec {
            Some(graphics_architecture) => map.insert(spec.name.clone(), graphics_architecture),
            None => None
        };
    }
    return map;
    
}

#[axum::debug_handler]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<Protobuf<proto_specdb::query::GraphicsArchitecture>, StatusCode> {
    match state.query_state.protobuf_graphics_architecture_hashmap.get(&name) {
        Some(value) => Ok(Protobuf(value.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}
