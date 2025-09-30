use crate::AppState;
use std::sync::Arc;

use axum::{extract::{Path, State}, Json};
use regex::RegexBuilder;
use specdb::{SpecDb, SpecDbStruct};

#[derive(Clone)]
pub struct PreProcessedState {
    pub name: String,
    pub result: SearchResult,
}

#[derive(serde::Serialize, Clone)]
pub struct SearchResult {
    name: String,
    human_name: String,
}

pub fn get_state(specdb: &SpecDb) -> Vec<PreProcessedState>
{
    let mut result = Vec::<PreProcessedState>::new();
    for spec in &specdb.files {
        result.push(PreProcessedState { name: strip_string(&spec.name), result: SearchResult { name: spec.name.clone(), human_name: spec.name.clone() } } ); 
    }
    return result;
    
}

pub async fn search_handler(
    State(state): State<Arc<AppState>>,
    Path(query): Path<String>
) -> Json<Vec<SearchResult>>
{
    // could pre-process the state.spec_db.files to have a stripped version of the name for searching
    let query_stripped = strip_string(&query);


    let mut result = Vec::<SearchResult>::new();
    for spec in &state.query_state.stripped_names {
        // if re.is_match(&spec.name) {
        if spec.name.contains(&query_stripped) {
            result.push(spec.result.clone());
        }
    }
    return Json(result);
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
