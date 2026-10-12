use std::fmt::Display;

use crate::geometry::Coordinate;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum JsError {
  CellOrdinalOutOfBounds(u32),
  EmptyOrdinal(u32),
  CoordinateOutOfBounds(Coordinate),
  UnrecognizedTypeId(String),
  InvalidValue(&'static str),
  InvalidCommanderIndex(u32),
  InvalidKind(u32, &'static str),
  FieldLabelNotFound(u32),
}

impl Display for JsError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      JsError::CellOrdinalOutOfBounds(ord) => write!(f, "Cell ordinal out of bounds: {ord}"),
      JsError::EmptyOrdinal(ord) => write!(f, "Cannot operate on empty ordinal: {ord}"),
      JsError::CoordinateOutOfBounds(coord) => {
        write!(f, "Coordinate out of bounds: ({}, {})", coord.x, coord.y)
      }
      JsError::UnrecognizedTypeId(type_id) => {
        write!(f, "Unrecognized type ID: {type_id}")
      }
      JsError::InvalidValue(msg) => write!(f, "Invalid value: {msg}"),
      JsError::InvalidCommanderIndex(index) => {
        write!(f, "Invalid commander index: {index}")
      }
      JsError::InvalidKind(kind, msg) => {
        write!(f, "Invalid kind: {kind}, {msg}")
      }
      JsError::FieldLabelNotFound(ord) => write!(f, "Field label not found: {ord}"),
    }
  }
}

impl From<JsError> for wasm_bindgen::JsValue {
  fn from(value: JsError) -> Self {
    wasm_bindgen::JsValue::from_str(&value.to_string())
  }
}
