use crate::AppState;
use std::sync::Arc;

use axum::{extract::{Path, State}, Json};
use regex::RegexBuilder;
use specdb::{SpecDb, SpecDbStruct};


#[derive(serde::Serialize)]
pub struct SearchResult {
    name: String,
    human_name: String,
}

pub async fn search_handler(
    State(state): State<Arc<AppState>>,
    Path(query): Path<String>
) -> Json<Vec<SearchResult>>
{
    // let re = RegexBuilder::new(&query)
    //     .case_insensitive(true)
    //     .build()
    //     .unwrap();
    // move away from regex
    let query_stripped = strip_string(query);


    let mut result = Vec::<SearchResult>::new();
    for spec in &state.spec_db.files {
        // if re.is_match(&spec.name) {
        if strip_string(spec.name.clone()).contains(&query_stripped) {
            result.push(SearchResult { name: spec.name.clone(), human_name: spec.name.clone() });
        }
    }
    return Json(result);
}

fn strip_string(string: String) -> String
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
