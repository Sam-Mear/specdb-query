use crate::AppState;
use std::sync::Arc;

use axum::{extract::{Path, State}, Json};
use regex::RegexBuilder;
use specdb::{SpecDb, SpecDbStruct};
use axum_extra::protobuf::Protobuf;

#[derive(Clone)]
pub struct PreProcessedState {
    pub stripped_search_name: String,
    pub result: SearchResult,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct SearchResultList {
    #[prost(message, repeated, tag="1")]
    pub results: Vec<SearchResult>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct SearchResult {
    #[prost(string, tag="1")]
    pub name: String,
    #[prost(enumeration="SpecType", tag="2")]
    pub spec_type: i32,
    #[prost(optional, string, tag="3")]
    pub human_name: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, prost::Enumeration)]
#[repr(i32)]
pub enum SpecType {
    Cpu = 0,
    Apu = 1,
    GraphicsCard = 2,
    CpuArchitecture = 3,
    ApuArchitecture = 4,
    GraphicsArchitecture = 5,
    GenericContainer = 6,
    Hidden = 7,
}

pub fn get_state(specdb: &SpecDb) -> Vec<PreProcessedState>
{
    let mut result = Vec::<PreProcessedState>::new();
    for spec in &specdb.files {
        result.push(PreProcessedState { 
            stripped_search_name: strip_string(&spec.name),
            result: SearchResult {
                name: spec.name.clone(),
                spec_type: match spec.part_type {
                    specdb::spectype::Type::Cpu(_) => SpecType::Cpu as i32,
                    specdb::spectype::Type::Apu(_) => SpecType::Apu as i32,
                    specdb::spectype::Type::GraphicsCard(_) => SpecType::GraphicsCard as i32,
                    specdb::spectype::Type::CpuArchitecture(_) => SpecType::CpuArchitecture as i32,
                    specdb::spectype::Type::ApuArchitecture(_) => SpecType::ApuArchitecture as i32,
                    specdb::spectype::Type::GraphicsArchitecture(_) => SpecType::GraphicsArchitecture as i32,
                    specdb::spectype::Type::GenericContainer(_) => SpecType::GenericContainer as i32,
                    specdb::spectype::Type::Hidden(_) => SpecType::Hidden as i32,
                },
                human_name: spec.human_name.clone()
            }
        }); 
    }
    return result;
    
}

#[axum::debug_handler]
pub async fn search_handler(
    State(state): State<Arc<AppState>>,
    Path(query): Path<String>
) -> Protobuf<SearchResultList>
{
    // support optional type filter by appending ":<type>" to the path query.
    // Examples:
    //   "ryzen"            -> search all types for "ryzen"
    //   "ryzen:Cpu"        -> only Cpu results for "ryzen"
    //   "ryzen:2"          -> only spec_type == 2 results for "ryzen"
    let mut query_term = query.clone();
    let mut filter_type: Option<i32> = None;

    if let Some(idx) = query.rfind(':') {
        let (left, right_with_colon) = query.split_at(idx);
        let right = &right_with_colon[1..]; // skip the ':'
        if !right.is_empty() {
            // try numeric parse first, then match common enum names (case-insensitive)
            if let Ok(n) = right.parse::<i32>() {
                filter_type = Some(n);
                query_term = left.to_string();
            } else {
                match right.to_ascii_lowercase().as_str() {
                    "cpu" => { filter_type = Some(SpecType::Cpu as i32); query_term = left.to_string(); }
                    "apu" => { filter_type = Some(SpecType::Apu as i32); query_term = left.to_string(); }
                    "graphicscard" | "graphics_card" | "gpu" | "graphics" => {
                        filter_type = Some(SpecType::GraphicsCard as i32); query_term = left.to_string();
                    }
                    "cpuarchitecture" | "cpu_architecture" => {
                        filter_type = Some(SpecType::CpuArchitecture as i32); query_term = left.to_string();
                    }
                    "apuarchitecture" | "apu_architecture" => {
                        filter_type = Some(SpecType::ApuArchitecture as i32); query_term = left.to_string();
                    }
                    "graphicsarchitecture" | "graphics_architecture" => {
                        filter_type = Some(SpecType::GraphicsArchitecture as i32); query_term = left.to_string();
                    }
                    "genericcontainer" | "generic_container" => {
                        filter_type = Some(SpecType::GenericContainer as i32); query_term = left.to_string();
                    }
                    "hidden" => { filter_type = Some(SpecType::Hidden as i32); query_term = left.to_string(); }
                    _ => { /* not a type specifier; treat entire query as search term */ }
                }
            }
        }
    }

    let query_stripped = strip_string(&query_term);

    let mut result = Vec::<SearchResult>::new();
    for spec in &state.query_state.stripped_names_protobuf {
        if spec.stripped_search_name.contains(&query_stripped) {
            if let Some(ft) = filter_type {
                if spec.result.spec_type == ft {
                    result.push(spec.result.clone());
                }
            } else {
                result.push(spec.result.clone());
            }
        }
    }

    return Protobuf(SearchResultList { results: result });
}

fn strip_string(string: &str) -> String
{
    string
        .chars()
        .filter_map(|c| {
            let c = c.to_ascii_lowercase();
            if c.is_alphanumeric() {
                Some(c)
            } else {
                None
            }
        })
        .collect()

}
