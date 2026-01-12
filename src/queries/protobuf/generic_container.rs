use crate::{AppState, proto_specdb::{self, query::Extra}};
use std::{collections::HashMap, sync::Arc};

use axum::{extract::{Path, State}, http::StatusCode};
use rapidhash::{HashMapExt, RapidHashMap};
use specdb::{SpecDb, spectype::Type};
use axum_extra::protobuf::Protobuf;

pub fn get_state(specdb: &SpecDb) -> RapidHashMap<String, proto_specdb::query::GenericContainer>
{
    let mut map = RapidHashMap::<String, proto_specdb::query::GenericContainer>::new();
    for spec in &specdb.files {
        let proto_spec = match &spec.part_type {
            Type::Cpu(_cpu) => None,
            Type::Apu(_apu) => None,
            Type::GraphicsCard(_graphics_card) => None,
            Type::CpuArchitecture(_cpu_architecture) => None,
            Type::ApuArchitecture(_apu_architecture) => None,
            Type::GraphicsArchitecture(_graphics_architecture) => None,
            Type::GenericContainer(generic_container) => Some(proto_specdb::query::GenericContainer {
                sections: generic_container.sections.iter().map(
                    |section| proto_specdb::query::Section{
                        header: section.header.clone(),
                        members: section.members.clone(),
                        extras: HashMap::<String, Extra>::new(),
                    }).collect(),
                top_header: generic_container.top_header.clone(),
            }),
            Type::Hidden(_inherit_data) => None,
        };
        match proto_spec {
            Some(generic_container) => map.insert(spec.name.clone(), generic_container),
            None => None
        };
    }
    return map;
    
}

#[axum::debug_handler]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<Protobuf<proto_specdb::query::GenericContainer>, StatusCode> {
    match state.query_state.protobuf_generic_container_hashmap.get(&name) {
        Some(value) => Ok(Protobuf(value.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}
