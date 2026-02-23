use std::sync::Arc;
use directories::ProjectDirs;
use specdb_query::{AppState, get_query_state, queries::{full_specs, search}};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Schema, http::GraphiQLSource};
use async_graphql_axum::GraphQL;
use axum::{
    Json, Router, extract::State, response::{self, IntoResponse}, routing::{get, post}
};
use specdb::{get_spec_db, SpecDb, SpecDbStruct};
use tower_http::trace::TraceLayer;
use yaml_rust2::YamlLoader;

pub struct QueryRoot;

struct Configuration {
    spec_db_path: String,
    allow_extras: bool,
}

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
    let config = read_config();

    let spec_db = get_spec_db(config.spec_db_path);
    let query_state = get_query_state(&spec_db);
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();
    let shared_state = Arc::new(AppState { spec_db: spec_db.clone(), query_state, allow_extras: config.allow_extras });
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(spec_db)
        .finish();
    
    // println!("Files parsed total: {}", spec_db.files.iter().count());

    // build our application with a single route
    let app = Router::new()
        // .route("/graphql", get(graphiql).post_service(GraphQL::new(schema)))
        .route("/", get(full_specs::handler_root).with_state(shared_state.clone()))// JSON
        .route("/v1/search/{query}", get(search::search_handler).with_state(shared_state.clone()))// JSON
        .route("/v1/spec/{name}", get(full_specs::handler).with_state(shared_state.clone()))// JSON 
        .route("/v1/protobuf/search/{query}", get(specdb_query::queries::protobuf::search::search_handler).with_state(shared_state.clone())) // PROTOBUF. See proto file.
        .route("/v1/protobuf/search_full_specs/{query}", get(specdb_query::queries::protobuf::search_result_full::search_handler).with_state(shared_state.clone())) // PROTOBUF. See proto file.
        .route("/v1/protobuf/cpu/{query}", get(specdb_query::queries::protobuf::cpu::handler).with_state(shared_state.clone())) // PROTOBUF. See proto file.
        .route("/v1/protobuf/graphics_card/{query}", get(specdb_query::queries::protobuf::graphics_card::handler).with_state(shared_state.clone())) // PROTOBUF. See proto file.
        .route("/v1/protobuf/apu/{query}", get(specdb_query::queries::protobuf::apu::handler).with_state(shared_state.clone())) // PROTOBUF. See proto file.
        .route("/v1/protobuf/cpu_architecture/{query}", get(specdb_query::queries::protobuf::cpu_architecture::handler).with_state(shared_state.clone())) // PROTOBUF. See proto file.
        .route("/v1/protobuf/graphics_architecture/{query}", get(specdb_query::queries::protobuf::graphics_architecture::handler).with_state(shared_state.clone())) // PROTOBUF. See proto file.
        .route("/v1/protobuf/apu_architecture/{query}", get(specdb_query::queries::protobuf::apu_architecture::handler).with_state(shared_state.clone())) // PROTOBUF. See proto file.
        .route("/v1/protobuf/generic_container/{query}", get(specdb_query::queries::protobuf::generic_container::handler).with_state(shared_state.clone())) // PROTOBUF. See proto file.
        .route("/v1/protobuf/extra", post(specdb_query::api::protobuf::extra::handler).with_state(shared_state.clone())) // PROTOBUF. See proto file.
        .route("/v1/protobuf/extra/{spec_name}", get(specdb_query::api::protobuf::extra::get_handler).with_state(shared_state.clone())) // PROTOBUF. See proto file.
        .route("/v1/protobuf/extra/export/{spec_name}", get(specdb_query::api::protobuf::extra::export_handler).with_state(shared_state.clone())) // PROTOBUF. See proto file.
        .route("/v1/protobuf/extra/import/{spec_name}", post(specdb_query::api::protobuf::extra::import_handler).with_state(shared_state.clone())) // PROTOBUF. See proto file.
        .route("/v1/protobuf/extra/export_all", get(specdb_query::api::protobuf::extra::export_all_handler).with_state(shared_state.clone())) // PROTOBUF. See proto file.
        .route("/v1/protobuf/extra/import_all", post(specdb_query::api::protobuf::extra::import_all_handler).with_state(shared_state.clone())) // PROTOBUF. See proto file.
        .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 8082
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8082").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn read_config() -> Configuration
{
    if let Some(proj_dirs) = ProjectDirs::from("info", "SpecDB",  "SpecDB Query API") {
        let config = proj_dirs.config_dir();
        if !config.exists() {
            std::fs::create_dir_all(config).unwrap();
        }
        let config_file = config.join("config.yaml");
        if !config_file.exists() {
            std::fs::write(&config_file, "").unwrap();
        }

        let yaml = YamlLoader::load_from_str(std::fs::read_to_string(&config_file).unwrap().as_str()).unwrap();
        if yaml.len() == 0 {
            panic!("Config file at {} is empty", config_file.display());
        }

        let spec_db_path = yaml[0]["spec_db_path"].as_str()
            .expect(format!("spec_db_path not found in config file at {}", config_file.display()).as_str()).to_string();

        // allow_extras defaults to false when not present
        let allow_extras = yaml[0]["allow_extras"].as_bool().unwrap_or(false);

        return Configuration {
            spec_db_path,
            allow_extras,
        };
    }
    panic!("Could not determine configuration directory");
}