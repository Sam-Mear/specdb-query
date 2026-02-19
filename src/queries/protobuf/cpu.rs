use crate::{AppState, proto_specdb};
use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode};
use rapidhash::{HashMapExt, RapidHashMap};
use specdb::{SpecDb, spectype::Type};
use axum_extra::protobuf::Protobuf;
use std::collections::HashMap;
use crate::proto_specdb::query::{SectionExtras};

pub fn get_state(specdb: &SpecDb) -> RapidHashMap<String, proto_specdb::query::Cpu>
{
    let mut map = RapidHashMap::<String, proto_specdb::query::Cpu>::new();
    for spec in &specdb.files {
        let proto_spec = match &spec.part_type {
            Type::Cpu(cpu) => Some(proto_specdb::query::Cpu {
                core_count: cpu.core_count.0 as u32,
                thread_count: cpu.thread_count.0 as u32,
                base_frequency: cpu.base_frequency.0.clone(),
                tdp: cpu.tdp.0.clone(),
                boost_frequency: match cpu.boost_frequency.clone() { Some(value) => Some(value.0), None => None},
                xfr_frequency: match cpu.xfr_frequency.clone() { Some(value) => Some(value.0), None => None},
                socket: match cpu.socket.clone() { Some(value) => Some(value.0), None => None},
                stepping: match cpu.stepping.clone() { Some(value) => Some(value.0), None => None},
                l1_cache_data: match cpu.l1_cache_data.clone() { Some(value) => Some(value.0), None => None},
                l1_cache_instruction: match cpu.l1_cache_instruction.clone() { Some(value) => Some(value.0), None => None},
                l2_cache_total: match cpu.l2_cache_total.clone() { Some(value) => Some(value.0), None => None},
                l3_cache_total: match cpu.l3_cache_total.clone() { Some(value) => Some(value.0), None => None},
                memory_type: match cpu.memory_type.clone() { Some(value) => Some(value.0), None => None},
                pcie_5_0_lanes: match cpu.pcie_5_0_lanes.clone() { Some(value) => Some(value.0), None => None},
                pcie_4_0_lanes: match cpu.pcie_4_0_lanes.clone() { Some(value) => Some(value.0), None => None},
                pcie_3_0_lanes: match cpu.pcie_3_0_lanes.clone() { Some(value) => Some(value.0), None => None},
                pcie_2_0_lanes: match cpu.pcie_2_0_lanes.clone() { Some(value) => Some(value.0), None => None},
                pcie_1_0_lanes: match cpu.pcie_1_0_lanes.clone() { Some(value) => Some(value.0), None => None},
                avx_sse_mmx: match cpu.avx_sse_mmx.clone() { Some(value) => Some(value.0), None => None},
                fma4: match cpu.fma4.clone() { Some(value) => Some(value.0), None => None},
                fma3: match cpu.fma3.clone() { Some(value) => Some(value.0), None => None},
                bmi: match cpu.bmi.clone() { Some(value) => Some(value.0), None => None},
                aes: match cpu.aes.clone() { Some(value) => Some(value.0), None => None},
                sha: match cpu.sha.clone() { Some(value) => Some(value.0), None => None},
                other_extensions: cpu.other_extensions.clone().map(|m| m.0).unwrap_or_default(),
                unlocked: match cpu.unlocked.clone() { Some(value) => Some(value.0), None => None},
                xfr_support: match cpu.xfr_support.clone() { Some(value) => Some(value.0), None => None},
                max_memory_channels: match cpu.max_memory_channels.clone() { Some(value) => Some(value.0), None => None},
                max_memory_frequency: match cpu.max_memory_frequency.clone() { Some(value) => Some(value.0), None => None},
                compatable_chipsets: cpu.compatable_chipsets.clone().map(|m| m.0).unwrap_or_default(),
                performance_core_base_frequency: match cpu.performance_core_base_frequency.clone() { Some(value) => Some(value.0), None => None},
                efficient_core_base_frequency: match cpu.efficient_core_base_frequency.clone() { Some(value) => Some(value.0), None => None},
                performance_core_boost_frequency: match cpu.performance_core_boost_frequency.clone() { Some(value) => Some(value.0), None => None},
                efficient_core_boost_frequency: match cpu.efficient_core_boost_frequency.clone() { Some(value) => Some(value.0), None => None},
                performance_core_count: match cpu.performance_core_count.clone() { Some(value) => Some(value.0 as u32), None => None},
                efficient_core_count: match cpu.efficient_core_count.clone() { Some(value) => Some(value.0 as u32), None => None},
                performance_thread_count: match cpu.performance_thread_count.clone() { Some(value) => Some(value.0 as u32), None => None},
                efficient_thread_count: match cpu.efficient_thread_count.clone() { Some(value) => Some(value.0 as u32), None => None},
                ctdp_support: match cpu.ctdp_support.clone() { Some(value) => Some(value.0), None => None},
                efficient_core_architecture: match cpu.efficient_core_architecture.clone() { Some(value) => Some(value.0), None => None},
                manufacturer: match cpu.manufacturer.clone() { Some(value) => Some(value.0), None => None},
                market: cpu.market.clone().map(|m| m.0).unwrap_or_default(),
                architecture: match cpu.architecture.clone() { Some(value) => Some(value.0), None => None},
                lithography: match cpu.lithography.clone() { Some(value) => Some(value.0), None => None},
                release_date: match cpu.release_date.clone() { Some(value) => Some(value.0), None => None},
                extras_by_section: HashMap::new(),
            }),
            Type::Apu(_apu) => None,
            Type::GraphicsCard(_graphics_card) => None,
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
) -> Result<Protobuf<proto_specdb::query::Cpu>, StatusCode> {
    match state.query_state.protobuf_cpu_hashmap.get(&name) {
        Some(value) => {
            // clone base cpu proto and attach extras
            let mut cpu = value.clone();

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
                cpu.extras_by_section = extras_by_section;
            }

            Ok(Protobuf(cpu))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}
