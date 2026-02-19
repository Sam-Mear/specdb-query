use std::sync::Arc;
use axum::{extract::State, http::StatusCode};
use axum::Json;
use serde_json::json;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

use crate::{AppState, proto_specdb};
use base64::{engine::general_purpose::STANDARD, Engine as _};

// Helpers to convert between prost_types::Struct/Value and serde_json::Value
fn prost_value_to_json(v: &prost_types::Value) -> serde_json::Value {
    use prost_types::value::Kind;
    match &v.kind {
        Some(Kind::NullValue(_)) => serde_json::Value::Null,
        Some(Kind::NumberValue(n)) => serde_json::Number::from_f64(*n).map_or(serde_json::Value::Null, |nn| serde_json::Value::Number(nn)),
        Some(Kind::StringValue(s)) => serde_json::Value::String(s.clone()),
        Some(Kind::BoolValue(b)) => serde_json::Value::Bool(*b),
        Some(Kind::StructValue(sv)) => prost_struct_to_json(sv),
        Some(Kind::ListValue(lv)) => serde_json::Value::Array(lv.values.iter().map(|it| prost_value_to_json(it)).collect()),
        None => serde_json::Value::Null,
    }
}

fn prost_struct_to_json(s: &prost_types::Struct) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for (k, v) in &s.fields {
        map.insert(k.clone(), prost_value_to_json(v));
    }
    serde_json::Value::Object(map)
}

fn json_to_prost_value(j: &serde_json::Value) -> Option<prost_types::Value> {
    use prost_types::{Value, value::Kind, Struct, ListValue};
    match j {
        serde_json::Value::Null => Some(Value { kind: Some(Kind::NullValue(0)) }),
        serde_json::Value::Bool(b) => Some(Value { kind: Some(Kind::BoolValue(*b)) }),
        serde_json::Value::Number(n) => n.as_f64().map(|f| Value { kind: Some(Kind::NumberValue(f)) }),
        serde_json::Value::String(s) => Some(Value { kind: Some(Kind::StringValue(s.clone())) }),
        serde_json::Value::Array(arr) => {
            let vals = arr.iter().filter_map(|it| json_to_prost_value(it)).collect();
            Some(Value { kind: Some(Kind::ListValue(ListValue { values: vals })) })
        }
        serde_json::Value::Object(obj) => {
            let mut fields = std::collections::BTreeMap::new();
            for (k, v) in obj.iter() {
                if let Some(pv) = json_to_prost_value(v) {
                    fields.insert(k.clone(), pv);
                }
            }
            Some(Value { kind: Some(Kind::StructValue(Struct { fields })) })
        }
    }
}

fn json_to_prost_struct(j: &serde_json::Value) -> Option<prost_types::Struct> {
    if let serde_json::Value::Object(obj) = j {
        let mut fields = std::collections::BTreeMap::new();
        for (k, v) in obj.iter() {
            if let Some(pv) = json_to_prost_value(v) {
                fields.insert(k.clone(), pv);
            }
        }
        Some(prost_types::Struct { fields })
    } else {
        None
    }
}


#[axum::debug_handler]
pub async fn export_all_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<JsonValue>, StatusCode> {
    let outer = state.query_state.extras_map.read().await;
    let mut top = serde_json::Map::new();

    for (spec_name, spec_map) in outer.iter() {
        let mut spec_obj = serde_json::Map::new();
        for (section, section_map) in spec_map.iter() {
            let mut section_obj = serde_json::Map::new();
            for (k, extra) in section_map.iter() {
                let mut e = serde_json::Map::new();
                e.insert("namespace".to_string(), json!(extra.namespace));

                if let Some(val) = extra.value.as_ref() {
                    use proto_specdb::query::extra::Value as Ev;
                    match val {
                        Ev::StringValue(s) => { e.insert("string_value".to_string(), json!(s)); }
                        Ev::IntValue(i) => { e.insert("int_value".to_string(), json!(i)); }
                        Ev::DoubleValue(d) => { e.insert("double_value".to_string(), json!(d)); }
                        Ev::BoolValue(b) => { e.insert("bool_value".to_string(), json!(b)); }
                        Ev::ObjectValue(o) => { e.insert("object_value".to_string(), prost_struct_to_json(o)); }
                        Ev::BytesValue(bv) => { e.insert("bytes_value_base64".to_string(), json!(STANDARD.encode(bv))); }
                    }
                }

                if !extra.double_list.is_empty() { e.insert("double_list".to_string(), json!(extra.double_list.clone())); }
                e.insert("version".to_string(), json!(extra.version));
                if let Some(ts) = &extra.updated_at { e.insert("updated_at".to_string(), json!({"seconds": ts.seconds, "nanos": ts.nanos})); }
                e.insert("source".to_string(), json!(extra.source));

                section_obj.insert(k.clone(), JsonValue::Object(e));
            }
            spec_obj.insert(section.clone(), JsonValue::Object(section_obj));
        }
        top.insert(spec_name.clone(), JsonValue::Object(spec_obj));
    }

    Ok(Json(JsonValue::Object(top)))
}


#[axum::debug_handler]
pub async fn import_all_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<JsonValue>,
) -> Result<StatusCode, StatusCode> {
    if !state.allow_extras { return Err(StatusCode::FORBIDDEN); }
    let mut outer = state.query_state.extras_map.write().await;
    let map = match payload.as_object() { Some(m) => m, None => return Err(StatusCode::BAD_REQUEST) };

    for (spec_name, spec_val) in map.iter() {
        let spec_entry = outer.entry(spec_name.clone()).or_insert_with(HashMap::new);
        if let Some(sections) = spec_val.as_object() {
            for (section_name, section_val) in sections.iter() {
                let section_map = spec_entry.entry(section_name.clone()).or_insert_with(HashMap::new);
                if let Some(kv) = section_val.as_object() {
                    for (k, v) in kv.iter() {
                        let mut extra = proto_specdb::query::Extra::default();
                        if let Some(ns) = v.get("namespace").and_then(|x| x.as_str()) { extra.namespace = ns.to_string(); }
                        use proto_specdb::query::extra::Value as Ev;
                        if let Some(s) = v.get("string_value").and_then(|x| x.as_str()) { extra.value = Some(Ev::StringValue(s.to_string())); }
                        else if let Some(i) = v.get("int_value").and_then(|x| x.as_i64()) { extra.value = Some(Ev::IntValue(i)); }
                        else if let Some(d) = v.get("double_value").and_then(|x| x.as_f64()) { extra.value = Some(Ev::DoubleValue(d)); }
                        else if let Some(b) = v.get("bool_value").and_then(|x| x.as_bool()) { extra.value = Some(Ev::BoolValue(b)); }
                        else if let Some(objv) = v.get("object_value") { if let Some(struct_val) = json_to_prost_struct(objv) { extra.value = Some(Ev::ObjectValue(struct_val)); } }
                        else if let Some(bs) = v.get("bytes_value_base64").and_then(|x| x.as_str()) { if let Ok(decoded) = STANDARD.decode(bs) { extra.value = Some(Ev::BytesValue(decoded)); } }
                        if let Some(arr) = v.get("double_list").and_then(|x| x.as_array()) { extra.double_list = arr.iter().filter_map(|vv| vv.as_f64()).collect(); }
                        if let Some(ver) = v.get("version").and_then(|x| x.as_u64()) { extra.version = ver; }
                        if let Some(obj) = v.get("updated_at").and_then(|x| x.as_object()) { if let (Some(s), Some(n)) = (obj.get("seconds").and_then(|x| x.as_i64()), obj.get("nanos").and_then(|x| x.as_i64())) { extra.updated_at = Some(prost_types::Timestamp { seconds: s, nanos: n as i32 }); } }
                        if let Some(src) = v.get("source").and_then(|x| x.as_str()) { extra.source = src.to_string(); }
                        section_map.insert(k.clone(), extra.clone());
                    }
                }
            }
        }
    }

    Ok(StatusCode::OK)
}
