use std::sync::Arc;
use specdb_query::{get_query_state, queries::search, AppState};
use axum::extract::Path;
use async_graphql::{Context, EmptyMutation, EmptySubscription, Schema, http::GraphiQLSource};
use async_graphql_axum::GraphQL;
use axum::{
    extract::State, routing::get, Json, Router, response::{self, IntoResponse}
};
use regex::{Regex, RegexBuilder};
use serde::{Serialize};
use specdb::{get_spec_db, spectype::Cpu, SpecDb, SpecDbStruct};
use tower_http::trace::TraceLayer;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn spec_db<'a>(
        &self,
        ctx: &Context<'a>,
    ) -> &'a Vec<SpecDbStruct> {
        return &ctx.data_unchecked::<SpecDb>().files;
    }

    // async fn cpus<'a>(
    //     &self,
    //     ctx: &Context<'a>,
    // ) -> &'a Vec<Cpu> {
        
    // }
    
}


async fn handler(
    State(state): State<Arc<AppState>>
) -> Json<SpecDb>
{
    return Json(state.spec_db.clone());
}


async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[tokio::main]
async fn main() {
    let spec_db = get_spec_db("/home/smear/personal/SpecDB/specs".to_string());
    let query_state = get_query_state(&spec_db);
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();
    let shared_state = Arc::new(AppState { spec_db, query_state });
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(get_spec_db("/home/smear/personal/SpecDB/specs".to_string()))
        .finish();
    
    // println!("Files parsed total: {}", spec_db.files.iter().count());

    // build our application with a single route
    let app = Router::new().route("/graphql", get(graphiql).post_service(GraphQL::new(schema)))
        .route("/", get(handler).with_state(shared_state.clone()))
        .route("/search/{query}", get(search::search_handler).with_state(shared_state.clone()))
        .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 8082
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8082").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}