use rapidhash::RapidHashMap;
use tokio::sync::RwLock;
use std::collections::HashMap;
use specdb::SpecDb;

use crate::queries::search::{PreProcessedState};

pub mod queries;
pub mod api;

pub mod proto_specdb {
    pub mod query {
        include!(concat!(env!("OUT_DIR"), "/specdb.query.rs"));
    }
}

pub struct QueryState {
    pub stripped_names: Vec<PreProcessedState>,
    pub spec_hash_map: RapidHashMap<String, specdb::SpecDbStruct>,
    pub stripped_names_protobuf: Vec<crate::queries::protobuf::search::PreProcessedState>,
    pub protobuf_cpu_hashmap: RapidHashMap<String, proto_specdb::query::Cpu>,
    pub protobuf_graphics_card_hashmap: RapidHashMap<String, proto_specdb::query::GraphicsCard>,
    pub protobuf_apu_hashmap: RapidHashMap<String, proto_specdb::query::Apu>,
    pub protobuf_cpu_architecture_hashmap: RapidHashMap<String, proto_specdb::query::CpuArchitecture>,
    pub protobuf_graphics_architecture_hashmap: RapidHashMap<String, proto_specdb::query::GraphicsArchitecture>,
    pub protobuf_apu_architecture_hashmap: RapidHashMap<String, proto_specdb::query::ApuArchitecture>,
    pub protobuf_generic_container_hashmap: RapidHashMap<String, proto_specdb::query::GenericContainer>,
    // extras_map: spec_name -> section_header -> key -> Extra
    pub extras_map: RwLock<HashMap<String, HashMap<String, HashMap<String, proto_specdb::query::Extra>>>>,
}

pub struct AppState {
    pub spec_db: SpecDb,
    pub query_state: QueryState,
    pub allow_extras: bool,
}

pub fn get_query_state(specdb: &SpecDb) -> QueryState
{
    return QueryState {
        stripped_names: crate::queries::search::get_state(&specdb),
        spec_hash_map: crate::queries::full_specs::get_state(&specdb),
        stripped_names_protobuf: crate::queries::protobuf::search::get_state(&specdb),
        protobuf_cpu_hashmap: crate::queries::protobuf::cpu::get_state(&specdb),
        protobuf_graphics_card_hashmap: crate::queries::protobuf::graphics_card::get_state(&specdb),
        protobuf_apu_hashmap: crate::queries::protobuf::apu::get_state(&specdb),
        protobuf_cpu_architecture_hashmap: crate::queries::protobuf::cpu_architecture::get_state(&specdb),
        protobuf_graphics_architecture_hashmap: crate::queries::protobuf::graphics_architecture::get_state(&specdb),
        protobuf_apu_architecture_hashmap: crate::queries::protobuf::apu_architecture::get_state(&specdb),
        protobuf_generic_container_hashmap: crate::queries::protobuf::generic_container::get_state(&specdb),
        extras_map: RwLock::new(HashMap::new()),
    }
}