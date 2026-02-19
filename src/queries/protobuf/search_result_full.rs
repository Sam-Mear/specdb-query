use crate::{AppState, proto_specdb::{self, query::{SearchResultFull, SearchResultFullList, search_result_full::FullSpecs}}};
use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode};
use axum_extra::protobuf::Protobuf;


#[axum::debug_handler]
pub async fn search_handler(
    State(state): State<Arc<AppState>>,
    Path(query): Path<String>
) -> Result<Protobuf<proto_specdb::query::SearchResultFullList>, StatusCode>
{
    // could pre-process the state.spec_db.files to have a stripped version of the name for searching
    let query_stripped = strip_string(&query);

    let mut result =  Vec::<SearchResultFull>::new(); // SearchResultFullList { search_result_full: };

    for spec in &state.query_state.stripped_names_protobuf {
        // if re.is_match(&spec.name) {
        if spec.stripped_search_name.contains(&query_stripped) {
            let full_result = match spec.result.spec_type() {
                super::search::SpecType::Cpu => SearchResultFull {
                    name: spec.result.name.clone(),
                    human_name: spec.result.human_name.clone(),
                    full_specs: match get_cpu(axum::extract::State(state.clone()), &spec.result.name) {
                        Some(value)=>Some(FullSpecs::Cpu(value)),
                        None => None,
                    },
                },
                super::search::SpecType::Apu => SearchResultFull {
                    name: spec.result.name.clone(),
                    human_name: spec.result.human_name.clone(),
                    full_specs: match get_apu(axum::extract::State(state.clone()), &spec.result.name) {
                        Some(value)=>Some(FullSpecs::Apu(value)),
                        None => None,
                    },
                },
                super::search::SpecType::GraphicsCard => SearchResultFull {
                    name: spec.result.name.clone(),
                    human_name: spec.result.human_name.clone(),
                    full_specs: match get_graphics_card(axum::extract::State(state.clone()), &spec.result.name) {
                        Some(value)=>Some(FullSpecs::GraphicsCard(value)),
                        None => None,
                    },
                },
                super::search::SpecType::CpuArchitecture => SearchResultFull {
                    name: spec.result.name.clone(),
                    human_name: spec.result.human_name.clone(),
                    full_specs: match get_cpu_architecture(axum::extract::State(state.clone()), &spec.result.name) {
                        Some(value)=>Some(FullSpecs::CpuArchitecture(value)),
                        None => None,
                    },
                },
                super::search::SpecType::ApuArchitecture => SearchResultFull {
                    name: spec.result.name.clone(),
                    human_name: spec.result.human_name.clone(),
                    full_specs: match get_apu_architecture(axum::extract::State(state.clone()), &spec.result.name) {
                        Some(value)=>Some(FullSpecs::ApuArchitecture(value)),
                        None => None,
                    },
                },
                super::search::SpecType::GraphicsArchitecture => SearchResultFull {
                    name: spec.result.name.clone(),
                    human_name: spec.result.human_name.clone(),
                    full_specs: match get_graphics_architecture(axum::extract::State(state.clone()), &spec.result.name) {
                        Some(value)=>Some(FullSpecs::GraphicsArchitecture(value)),
                        None => None,
                    },
                },
                super::search::SpecType::GenericContainer => SearchResultFull {
                    name: spec.result.name.clone(),
                    human_name: spec.result.human_name.clone(),
                    full_specs: match get_generic_container(axum::extract::State(state.clone()), &spec.result.name) {
                        Some(value)=>Some(FullSpecs::GenericContainer(value)),
                        None => None,
                    },
                },
                super::search::SpecType::Hidden => SearchResultFull {
                    name: spec.result.name.clone(),
                    human_name: spec.result.human_name.clone(),
                    full_specs: Some(FullSpecs::Hidden(true))
                },
            };
            result.push(full_result);
        }
    }
    return Ok(Protobuf(SearchResultFullList{ search_result_full: result}));
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

pub fn get_cpu(
    state: State<Arc<AppState>>,
    name: &String,
) -> Option<proto_specdb::query::Cpu> {
    match state.query_state.protobuf_cpu_hashmap.get(name) {
        Some(value) => Some(value.clone()),
        None => None,
    }
}

pub fn get_apu(
    state: State<Arc<AppState>>,
    name: &String,
) -> Option<proto_specdb::query::Apu> {
    match state.query_state.protobuf_apu_hashmap.get(name) {
        Some(value) => Some(value.clone()),
        None => None,
    }
}

pub fn get_graphics_card(
    state: State<Arc<AppState>>,
    name: &String,
) -> Option<proto_specdb::query::GraphicsCard> {
    match state.query_state.protobuf_graphics_card_hashmap.get(name) {
        Some(value) => Some(value.clone()),
        None => None,
    }
}

pub fn get_cpu_architecture(
    state: State<Arc<AppState>>,
    name: &String,
) -> Option<proto_specdb::query::CpuArchitecture> {
    match state.query_state.protobuf_cpu_architecture_hashmap.get(name) {
        Some(value) => Some(value.clone()),
        None => None,
    }
}
pub fn get_apu_architecture(
    state: State<Arc<AppState>>,
    name: &String,
) -> Option<proto_specdb::query::ApuArchitecture> {
    match state.query_state.protobuf_apu_architecture_hashmap.get(name) {
        Some(value) => Some(value.clone()),
        None => None,
    }
}
pub fn get_graphics_architecture(
    state: State<Arc<AppState>>,
    name: &String,
) -> Option<proto_specdb::query::GraphicsArchitecture> {
    match state.query_state.protobuf_graphics_architecture_hashmap.get(name) {
        Some(value) => Some(value.clone()),
        None => None,
    }
}
pub fn get_generic_container(
    state: State<Arc<AppState>>,
    name: &String,
) -> Option<proto_specdb::query::GenericContainer> {
    match state.query_state.protobuf_generic_container_hashmap.get(name) {
        Some(value) => Some(value.clone()),
        None => None,
    }
}
