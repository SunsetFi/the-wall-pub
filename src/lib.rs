use wasm_bindgen::prelude::wasm_bindgen;

mod alignment;
mod blob_field;
mod combat;
mod commander_entity;
mod direction;
mod entity;
mod entity_slot;
mod fluid;
mod geometry;
mod grid_collection;
mod js_error;
mod js_log;
mod material;
mod micro_entity;
mod permeability;
mod quantity;
mod sim;
mod structure_entity;
mod utils;
mod world;

pub use js_error::JsError;
pub use sim::Sim;

use crate::{
  commander_entity::CommanderKind, micro_entity::MicroKind, structure_entity::StructureKind,
};

#[wasm_bindgen(start)]
fn init() {
  utils::set_panic_hook();
}

#[wasm_bindgen]
pub fn memory_js() -> wasm_bindgen::JsValue {
  wasm_bindgen::memory()
}

#[wasm_bindgen]
pub fn micro_key_for_type(type_id: &str) -> Result<MicroKind, js_error::JsError> {
  MicroKind::from_type_id(type_id)
    .ok_or_else(|| js_error::JsError::UnrecognizedTypeId(type_id.to_string()))
}

#[wasm_bindgen]
pub fn commander_key_for_type(type_id: &str) -> Result<CommanderKind, js_error::JsError> {
  CommanderKind::from_type_id(type_id)
    .ok_or_else(|| js_error::JsError::UnrecognizedTypeId(type_id.to_string()))
}

#[wasm_bindgen]
pub fn structure_key_for_type(type_id: &str) -> Result<StructureKind, js_error::JsError> {
  StructureKind::from_type_id(type_id)
    .ok_or_else(|| js_error::JsError::UnrecognizedTypeId(type_id.to_string()))
}
