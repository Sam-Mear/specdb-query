use std::sync::Arc;

use axum::{extract::{Path, State}, Json};
use rapidhash::{HashMapExt, RapidHashMap};
use specdb::{SpecDb, SpecDbStruct};

use crate::AppState;


pub fn get_state(specdb: &SpecDb) -> RapidHashMap<String, SpecDbStruct>
{
    let mut map = RapidHashMap::<String, SpecDbStruct>::new();
    for spec in &specdb.files {
        map.insert(spec.name.clone(), spec.clone());
    }
    return map;
    
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>
) -> Json<Option<SpecDbStruct>>
{
    return Json(state.query_state.spec_hash_map.get(&name).cloned());
}

pub async fn handler_root(
    State(state): State<Arc<AppState>>
) -> Json<Option<SpecDbStruct>>
{
    return Json(state.query_state.spec_hash_map.get("Root").cloned());
}
