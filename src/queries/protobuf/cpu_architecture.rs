use crate::{AppState, proto_specdb::{self, query::{Extra}}};
use std::{collections::HashMap, sync::Arc};

use axum::{extract::{Path, State}, http::StatusCode};
use rapidhash::{HashMapExt, RapidHashMap};
use specdb::{SpecDb, spectype::Type};
use axum_extra::protobuf::Protobuf;

pub fn get_state(specdb: &SpecDb) -> RapidHashMap<String, proto_specdb::query::CpuArchitecture>
{
    let mut map = RapidHashMap::<String, proto_specdb::query::CpuArchitecture>::new();
    for spec in &specdb.files {
        let proto_spec = match &spec.part_type {
            Type::Cpu(_cpu) => None,
            Type::Apu(_apu) => None,
            Type::GraphicsCard(_graphics_card) => None,
            Type::CpuArchitecture(cpu_architecture) => Some(proto_specdb::query::CpuArchitecture {
                sections: cpu_architecture.sections.iter().map(
                    |section| proto_specdb::query::Section{
                        header: section.header.clone(),
                        members: section.members.clone(),
                        extras: HashMap::<String, Extra>::new(),
                    }).collect(),
                lithography: cpu_architecture.lithography.0.clone(),
                release_date: cpu_architecture.release_date.0.clone(),
                sockets: cpu_architecture.sockets.0.clone() 
            }),
            Type::ApuArchitecture(_apu_architecture) => None,
            Type::GraphicsArchitecture(_graphics_architecture) => None,
            Type::GenericContainer(_generic_container) => None,
            Type::Hidden(_inherit_data) => None,
        };
        match proto_spec {
            Some(cpu_architecture) => map.insert(spec.name.clone(), cpu_architecture),
            None => None
        };
    }
    return map;
    
}

#[axum::debug_handler]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<Protobuf<proto_specdb::query::CpuArchitecture>, StatusCode> {
    match state.query_state.protobuf_cpu_architecture_hashmap.get(&name) {
        Some(value) => Ok(Protobuf(value.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}
