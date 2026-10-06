use std::fmt::Display;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum JsError {
  SimNotInitialized,
  OutOfBounds(u32),
  UnrecognizedTypeId(String),
  UnrecognizedId(String),
  InvalidValue(&'static str),
  InvalidCommanderIndex(u32),
  InvalidKind(&'static str),
}

impl Display for JsError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      JsError::SimNotInitialized => write!(f, "Simulation not initialized"),
      JsError::OutOfBounds(ord) => write!(f, "Out of bounds: {ord}"),
      JsError::UnrecognizedTypeId(type_id) => {
        write!(f, "Unrecognized type ID: {type_id}")
      }
      JsError::UnrecognizedId(id) => write!(f, "Unrecognized ID: {id}"),
      JsError::InvalidValue(msg) => write!(f, "Invalid value: {msg}"),
      JsError::InvalidCommanderIndex(index) => {
        write!(f, "Invalid commander index: {index}")
      }
      JsError::InvalidKind(kind) => {
        write!(f, "Invalid kind: {kind}")
      }
    }
  }
}

impl From<JsError> for wasm_bindgen::JsValue {
  fn from(value: JsError) -> Self {
    wasm_bindgen::JsValue::from_str(&value.to_string())
  }
}
