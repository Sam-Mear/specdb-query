pub mod handlers;
pub mod json;

// Re-export common handlers at the `extra` module level so existing callers
// (e.g. `specdb_query::api::protobuf::extra::handler`) keep working.
pub use handlers::*;
pub use json::{export_all_handler, import_all_handler};
