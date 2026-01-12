use crate::{AppState, proto_specdb};
use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode};
use rapidhash::{HashMapExt, RapidHashMap};
use std::collections::HashMap;
use crate::proto_specdb::query::SectionExtras;
use specdb::{SpecDb, spectype::Type};
use axum_extra::protobuf::Protobuf;

pub fn get_state(specdb: &SpecDb) -> RapidHashMap<String, proto_specdb::query::GraphicsCard>
{
    let mut map = RapidHashMap::<String, proto_specdb::query::GraphicsCard>::new();
    for spec in &specdb.files {
        let proto_spec = match &spec.part_type {
            Type::Cpu(_cpu) => None,
            Type::Apu(_apu) => None,
                Type::GraphicsCard(graphics_card) => Some(proto_specdb::query::GraphicsCard{
                vram_capacity: graphics_card.vram_capacity.0.clone(),
                shader_processor_count: graphics_card.shader_processor_count.0.clone(),
                gpu_base_frequency: graphics_card.gpu_base_frequency.0.clone(),
                manufacturer: match graphics_card.manufacturer.clone() { Some(value) => Some(value.0.clone()), None => None},
                vendor: match graphics_card.vendor.clone() { Some(value) => Some(value.0.clone()), None => None},
                market: graphics_card.market.clone().map(|m| m.0).unwrap_or_default(),
                architecture: match graphics_card.architecture.clone() { Some(value) => Some(value.0.clone()), None => None},
                lithography: match graphics_card.lithography.clone() { Some(value) => Some(value.0.clone()), None => None},
                release_date: match graphics_card.release_date.clone() { Some(value) => Some(value.0.clone()), None => None},
                direct_x_support: match graphics_card.direct_x_support.clone() { Some(value) => Some(value.0.clone()), None => None},
                open_gl_support: match graphics_card.open_gl_support.clone() { Some(value) => Some(value.0.clone()), None => None},
                open_cl_support: match graphics_card.open_cl_support.clone() { Some(value) => Some(value.0.clone()), None => None},
                vulkan_support: match graphics_card.vulkan_support.clone() { Some(value) => Some(value.0.clone()), None => None},
                vram_frequency: match graphics_card.vram_frequency.clone() { Some(value) => Some(value.0.clone()), None => None},
                vram_type: match graphics_card.vram_type.clone() { Some(value) => Some(value.0.clone()), None => None},
                vram_bandwidth: match graphics_card.vram_bandwidth.clone() { Some(value) => Some(value.0.clone()), None => None},
                vram_bus_width: match graphics_card.vram_bus_width.clone() { Some(value) => Some(value.0.clone()), None => None},
                render_output_unit_count: match graphics_card.render_output_unit_count.clone() { Some(value) => Some(value.0 as u32), None => None},
                texture_mapping_unit_count: match graphics_card.texture_mapping_unit_count.clone() { Some(value) => Some(value.0 as u32), None => None},
                die_size: match graphics_card.die_size.clone() { Some(value) => Some(value.0.clone()), None => None},
                tdp: match graphics_card.tdp.clone() { Some(value) => Some(value.0.clone()), None => None},
                gpu: match graphics_card.gpu.clone() { Some(value) => Some(value.0.clone()), None => None},
                gpu_variant: match graphics_card.gpu_variant.clone() { Some(value) => Some(value.0.clone()), None => None},
                gpu_model: match graphics_card.gpu_model.clone() { Some(value) => Some(value.0.clone()), None => None},
                hlsl_shader_model: match graphics_card.hlsl_shader_model.clone() { Some(value) => Some(value.0.clone()), None => None},
                gpu_boost_frequency: match graphics_card.gpu_boost_frequency.clone() { Some(value) => Some(value.0.clone()), None => None},
                fp32_compute: match graphics_card.fp32_compute.clone() { Some(value) => Some(value.0.clone()), None => None},
                fp64_compute: match graphics_card.fp64_compute.clone() { Some(value) => Some(value.0.clone()), None => None},
                slot_width: match graphics_card.slot_width.clone() { Some(value) => Some(value.0.clone()), None => None},
                outputs: graphics_card.outputs.clone().map(|m| m.0).unwrap_or_default(),
                power_connectors: graphics_card.power_connectors.clone().map(|m| m.0).unwrap_or_default(),
                length: match graphics_card.length.clone() { Some(value) => Some(value.0.clone()), None => None},
                height: match graphics_card.height.clone() { Some(value) => Some(value.0.clone()), None => None},
                width: match graphics_card.width.clone() { Some(value) => Some(value.0.clone()), None => None},
                ray_tracing_cores: match graphics_card.ray_tracing_cores.clone() { Some(value) => Some(value.0 as u32), None => None},
                tensor_cores: match graphics_card.tensor_cores.clone() { Some(value) => Some(value.0 as u32), None => None},
                hardware_accelerated_encoding: graphics_card.hardware_accelerated_encoding.clone().map(|m| m.0).unwrap_or_default(),
                hardware_accelerated_decoding: graphics_card.hardware_accelerated_decoding.clone().map(|m| m.0).unwrap_or_default(),
                module_count: match graphics_card.module_count.clone() { Some(value) => Some(value.0.clone()), None => None},
                pixel_shaders: match graphics_card.pixel_shaders.clone() { Some(value) => Some(value.0 as u32), None => None},
                maximum_vram_capacity: match graphics_card.maximum_vram_capacity.clone() { Some(value) => Some(value.0.clone()), None => None},
                max_displays: match graphics_card.max_displays.clone() { Some(value) => Some(value.0.clone()), None => None},
                crossfire_support: match graphics_card.crossfire_support.clone() { Some(value) => Some(value.0.clone()), None => None},
                free_sync_support: match graphics_card.free_sync_support.clone() { Some(value) => Some(value.0.clone()), None => None}
                ,
                extras_by_section: HashMap::new()
            }),
            Type::CpuArchitecture(_cpu_architecture) => None,
            Type::ApuArchitecture(_apu_architecture) => None,
            Type::GraphicsArchitecture(_graphics_architecture) => None,
            Type::GenericContainer(_generic_container) => None,
            Type::Hidden(_inherit_data) => None,
        };
        match proto_spec {
                Some(cpu) => map.insert(spec.name.clone(), cpu),
            None => None
        };
    }
    return map;
    
}

#[axum::debug_handler]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<Protobuf<proto_specdb::query::GraphicsCard>, StatusCode> {
    match state.query_state.protobuf_graphics_card_hashmap.get(&name) {
        Some(value) => {
            let mut card = value.clone();
            let outer = state.query_state.extras_map.read().await;
            if let Some(spec_map) = outer.get(&name) {
                let mut extras_by_section: HashMap<String, SectionExtras> = HashMap::new();
                for (section, section_map) in spec_map.iter() {
                    let mut s = SectionExtras { extras: HashMap::new() };
                    for (k, v) in section_map.iter() {
                        s.extras.insert(k.clone(), v.clone());
                    }
                    extras_by_section.insert(section.clone(), s);
                }
                card.extras_by_section = extras_by_section;
            }
            Ok(Protobuf(card))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}
