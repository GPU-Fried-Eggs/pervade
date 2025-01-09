use alloc::vec::Vec;

use serde::{Deserialize, Serialize};
use wamr_rust_sdk::value::WasmValue;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Value(#[serde(with = "WasmValueDef")] pub WasmValue);

impl Value {
    pub fn new(value: WasmValue) -> Self {
        Self(value)
    }

    pub fn encode(&self) -> Vec<u32> {
        self.0.encode()
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        Value(match &self.0 {
            WasmValue::Void => WasmValue::Void,
            WasmValue::I32(value) => WasmValue::I32(*value),
            WasmValue::I64(value) => WasmValue::I64(*value),
            WasmValue::F32(value) => WasmValue::F32(*value),
            WasmValue::F64(value) => WasmValue::F64(*value),
            WasmValue::V128(value) => WasmValue::V128(*value),
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(remote = "WasmValue")]
#[allow(dead_code)]
enum WasmValueDef {
    Void,
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    V128(i128),
}

impl From<WasmValueDef> for WasmValue {
    fn from(def: WasmValueDef) -> WasmValue {
        match def {
            WasmValueDef::Void => WasmValue::Void,
            WasmValueDef::I32(value) => WasmValue::I32(value),
            WasmValueDef::I64(value) => WasmValue::I64(value),
            WasmValueDef::F32(value) => WasmValue::F32(value),
            WasmValueDef::F64(value) => WasmValue::F64(value),
            WasmValueDef::V128(value) => WasmValue::V128(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_serialize_wasm_values() {
        let test_cases = vec![
            (WasmValue::Void, json!("Void")),
            (WasmValue::I32(42), json!({"I32": 42})),
            (WasmValue::I64(42), json!({"I64": 42})),
            (WasmValue::F32(42.0), json!({"F32": 42.0})),
            (WasmValue::F64(42.0), json!({"F64": 42.0})),
            (WasmValue::V128(42), json!({"V128": 42})),
        ];

        for (value, expected_json) in test_cases {
            let serialized = serde_json::to_string(&Value(value)).unwrap();
            assert_eq!(serialized, expected_json.to_string());
        }
    }

    #[test]
    fn test_deserialize_wasm_values() {
        let test_cases = vec![
            (json!("Void"), WasmValue::Void),
            (json!({"I32": 42}), WasmValue::I32(42)),
            (json!({"I64": 42}), WasmValue::I64(42)),
            (json!({"F32": 42.0}), WasmValue::F32(42.0)),
            (json!({"F64": 42.0}), WasmValue::F64(42.0)),
            (json!({"V128": 42}), WasmValue::V128(42)),
        ];

        for (json_val, expected_value) in test_cases {
            let json_str = json_val.to_string();
            let Value(deserialized) = serde_json::from_str(&json_str).unwrap();
            assert_eq!(deserialized, expected_value);
        }
    }
}
