use crate::{AppState, proto_specdb::{self}};
use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode};
use rapidhash::{HashMapExt, RapidHashMap};
use specdb::{SpecDb, SpecDbStruct, spectype::Type};
use axum_extra::protobuf::Protobuf;
use tracing::{debug, info};

pub fn get_state(specdb: &SpecDb) -> RapidHashMap<String, proto_specdb::query::CpuArchitecture>
{
    let mut map = RapidHashMap::<String, proto_specdb::query::CpuArchitecture>::new();
    for spec in &specdb.files {
        let proto_spec = match &spec.part_type {
            Type::Cpu(cpu) => None,
            Type::Apu(apu) => None,
            Type::GraphicsCard(graphics_card) => None,
            Type::CpuArchitecture(cpu_architecture) => Some(proto_specdb::query::CpuArchitecture {
                sections: cpu_architecture.sections.iter().map(|section| proto_specdb::query::Section{header: section.header.clone(), members: section.members.clone()}).collect(),
                lithography: cpu_architecture.lithography.0.clone(),
                release_date: cpu_architecture.release_date.0.clone(),
                sockets: cpu_architecture.sockets.0.clone() 
            }),
            Type::ApuArchitecture(apu_architecture) => None,
            Type::GraphicsArchitecture(graphics_architecture) => None,
            Type::GenericContainer(generic_container) => None,
            Type::Hidden(inherit_data) => None,
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
