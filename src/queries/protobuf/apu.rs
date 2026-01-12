use crate::{AppState, proto_specdb};
use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode};
use rapidhash::{HashMapExt, RapidHashMap};
use std::collections::HashMap;
use crate::proto_specdb::query::SectionExtras;
use specdb::{SpecDb, spectype::Type};
use axum_extra::protobuf::Protobuf;

pub fn get_state(specdb: &SpecDb) -> RapidHashMap<String, proto_specdb::query::Apu>
{
    let mut map = RapidHashMap::<String, proto_specdb::query::Apu>::new();
    for spec in &specdb.files {
        let proto_spec = match &spec.part_type {
            Type::Cpu(_cpu) => None,
                Type::Apu(apu) => Some(proto_specdb::query::Apu {
                core_count: apu.core_count.0 as u32,
                thread_count: apu.thread_count.0 as u32,
                base_frequency: apu.base_frequency.0.clone(),
                shader_processor_count: apu.shader_processor_count.clone().0,
                tdp: match apu.tdp.clone() { Some(value) => Some(value.0.clone()), None => None},
                boost_frequency: match apu.boost_frequency.clone() { Some(value) => Some(value.0), None => None},
                xfr_frequency: match apu.xfr_frequency.clone() { Some(value) => Some(value.0), None => None},
                socket: match apu.socket.clone() { Some(value) => Some(value.0), None => None},
                stepping: match apu.stepping.clone() { Some(value) => Some(value.0), None => None},
                l1_cache_data: match apu.l1_cache_data.clone() { Some(value) => Some(value.0), None => None},
                l1_cache_instruction: match apu.l1_cache_instruction.clone() { Some(value) => Some(value.0), None => None},
                l2_cache_total: match apu.l2_cache_total.clone() { Some(value) => Some(value.0), None => None},
                l3_cache_total: match apu.l3_cache_total.clone() { Some(value) => Some(value.0), None => None},
                memory_type: match apu.memory_type.clone() { Some(value) => Some(value.0), None => None},
                pcie_5_0_lanes: match apu.pcie_5_0_lanes.clone() { Some(value) => Some(value.0), None => None},
                pcie_4_0_lanes: match apu.pcie_4_0_lanes.clone() { Some(value) => Some(value.0), None => None},
                pcie_3_0_lanes: match apu.pcie_3_0_lanes.clone() { Some(value) => Some(value.0), None => None},
                pcie_2_0_lanes: match apu.pcie_2_0_lanes.clone() { Some(value) => Some(value.0), None => None},
                pcie_1_0_lanes: match apu.pcie_1_0_lanes.clone() { Some(value) => Some(value.0), None => None},
                avx_sse_mmx: match apu.avx_sse_mmx.clone() { Some(value) => Some(value.0), None => None},
                fma4: match apu.fma4.clone() { Some(value) => Some(value.0), None => None},
                fma3: match apu.fma3.clone() { Some(value) => Some(value.0), None => None},
                bmi: match apu.bmi.clone() { Some(value) => Some(value.0), None => None},
                aes: match apu.aes.clone() { Some(value) => Some(value.0), None => None},
                sha: match apu.sha.clone() { Some(value) => Some(value.0), None => None},
                other_extensions: apu.other_extensions.clone().map(|m| m.0).unwrap_or_default(),
                unlocked: match apu.unlocked.clone() { Some(value) => Some(value.0), None => None},
                xfr_support: match apu.xfr_support.clone() { Some(value) => Some(value.0), None => None},
                max_memory_channels: match apu.max_memory_channels.clone() { Some(value) => Some(value.0), None => None},
                max_memory_frequency: match apu.max_memory_frequency.clone() { Some(value) => Some(value.0), None => None},
                compatable_chipsets: apu.compatable_chipsets.clone().map(|m| m.0).unwrap_or_default(),
                performance_core_base_frequency: match apu.performance_core_base_frequency.clone() { Some(value) => Some(value.0), None => None},
                efficient_core_base_frequency: match apu.efficient_core_base_frequency.clone() { Some(value) => Some(value.0), None => None},
                performance_core_boost_frequency: match apu.performance_core_boost_frequency.clone() { Some(value) => Some(value.0), None => None},
                efficient_core_boost_frequency: match apu.efficient_core_boost_frequency.clone() { Some(value) => Some(value.0), None => None},
                performance_core_count: match apu.performance_core_count.clone() { Some(value) => Some(value.0 as u32), None => None},
                efficient_core_count: match apu.efficient_core_count.clone() { Some(value) => Some(value.0 as u32), None => None},
                performance_thread_count: match apu.performance_thread_count.clone() { Some(value) => Some(value.0 as u32), None => None},
                efficient_thread_count: match apu.efficient_thread_count.clone() { Some(value) => Some(value.0 as u32), None => None},
                ctdp_support: match apu.ctdp_support.clone() { Some(value) => Some(value.0), None => None},
                efficient_core_architecture: match apu.efficient_core_architecture.clone() { Some(value) => Some(value.0), None => None},
                manufacturer: match apu.manufacturer.clone() { Some(value) => Some(value.0), None => None},
                market: apu.market.clone().map(|m| m.0).unwrap_or_default(),
                architecture: match apu.architecture.clone() { Some(value) => Some(value.0), None => None},
                lithography: match apu.lithography.clone() { Some(value) => Some(value.0), None => None},
                release_date: match apu.release_date.clone() { Some(value) => Some(value.0), None => None},
                extras_by_section: HashMap::new(),
                gpu_base_frequency: match apu.gpu_base_frequency.clone() { Some(value) => Some(value.0), None => None },
                direct_x_support: match apu.direct_x_support.clone() { Some(value) => Some(value.0.clone()), None => None},
                open_gl_support: match apu.open_gl_support.clone() { Some(value) => Some(value.0.clone()), None => None},
                open_cl_support: match apu.open_cl_support.clone() { Some(value) => Some(value.0.clone()), None => None},
                vulkan_support: match apu.vulkan_support.clone() { Some(value) => Some(value.0.clone()), None => None},
                vram_type: match apu.vram_type.clone() { Some(value) => Some(value.0.clone()), None => None},
                render_output_unit_count: match apu.render_output_unit_count.clone() { Some(value) => Some(value.0 as u32), None => None},
                texture_mapping_unit_count: match apu.texture_mapping_unit_count.clone() { Some(value) => Some(value.0 as u32), None => None},
                gpu_model: match apu.gpu_model.clone() { Some(value) => Some(value.0.clone()), None => None},
                hlsl_shader_model: match apu.hlsl_shader_model.clone() { Some(value) => Some(value.0.clone()), None => None},
                gpu_boost_frequency: match apu.gpu_boost_frequency.clone() { Some(value) => Some(value.0.clone()), None => None},
                ray_tracing_cores: match apu.ray_tracing_cores.clone() { Some(value) => Some(value.0 as u32), None => None},
                tensor_cores: match apu.tensor_cores.clone() { Some(value) => Some(value.0 as u32), None => None},
                hardware_accelerated_encoding: apu.hardware_accelerated_encoding.clone().map(|m| m.0).unwrap_or_default(),
                hardware_accelerated_decoding: apu.hardware_accelerated_decoding.clone().map(|m| m.0).unwrap_or_default(),
                module_count: match apu.module_count.clone() { Some(value) => Some(value.0.clone()), None => None},
                pixel_shaders: match apu.pixel_shaders.clone() { Some(value) => Some(value.0 as u32), None => None},
                max_displays: match apu.max_displays.clone() { Some(value) => Some(value.0.clone()), None => None},
                crossfire_support: match apu.crossfire_support.clone() { Some(value) => Some(value.0.clone()), None => None},
                free_sync_support: match apu.free_sync_support.clone() { Some(value) => Some(value.0.clone()), None => None}
            }),
            Type::GraphicsCard(_graphics_card) => None,
            Type::CpuArchitecture(_cpu_architecture) => None,
            Type::ApuArchitecture(_apu_architecture) => None,
            Type::GraphicsArchitecture(_graphics_architecture) => None,
            Type::GenericContainer(_generic_container) => None,
            Type::Hidden(_inherit_data) => None,
        };
        match proto_spec {
            Some(apu) => map.insert(spec.name.clone(), apu),
            None => None
        };
    }
    return map;
    
}

#[axum::debug_handler]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<Protobuf<proto_specdb::query::Apu>, StatusCode> {
    match state.query_state.protobuf_apu_hashmap.get(&name) {
        Some(value) => {
            let mut apu = value.clone();
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
                apu.extras_by_section = extras_by_section;
            }
            Ok(Protobuf(apu))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}
