use specdb::SpecDb;

use crate::queries::search::{PreProcessedState};

pub mod queries;

pub struct QueryState {
    pub stripped_names: Vec<PreProcessedState>,
}

pub struct AppState {
    pub spec_db: SpecDb,
    pub query_state: QueryState,
}

pub fn get_query_state(specdb: &SpecDb) -> QueryState
{
    let stripped_names = crate::queries::search::get_state(&specdb);
    return QueryState { stripped_names };
}