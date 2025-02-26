use alloc::{string::String, vec::Vec};

use bincode::{config, error, Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::value::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum ServerMessage {
    InstallModule {
        module_name: String,
        module_data: Vec<u8>,
    },
    UninstallModule {
        module_name: String,
    },
    TaskAssignment {
        task_id: u64,
        module_name: String,
        function_name: String,
        #[bincode(with_serde)]
        inputs: Vec<Value>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum ClientMessage {
    TaskStatusUpdate {
        task_id: u64,
        status: TaskStatus,
    },
    TaskResult {
        task_id: u64,
        #[bincode(with_serde)]
        result: Value,
    },
}

impl ServerMessage {
    pub fn encode(data: &Self) -> Result<Vec<u8>, error::EncodeError> {
        Ok(bincode::encode_to_vec(data, config::standard())?)
    }

    pub fn decode(data: &[u8]) -> Result<Self, error::DecodeError> {
        Ok(bincode::decode_from_slice(data, config::standard())?.0)
    }
}

impl ClientMessage {
    pub fn encode(data: &Self) -> Result<Vec<u8>, error::EncodeError> {
        Ok(bincode::encode_to_vec(data, config::standard())?)
    }

    pub fn decode(data: &[u8]) -> Result<Self, error::DecodeError> {
        Ok(bincode::decode_from_slice(data, config::standard())?.0)
    }
}

#[cfg(test)]
mod tests {
    use wamr_rust_sdk::value::WasmValue;

    use super::*;
    use crate::value::Value;

    #[test]
    fn test_server_message_serde() {
        let msg = ServerMessage::InstallModule {
            module_name: "test_module".into(),
            module_data: vec![1, 2, 3, 4],
        };
        let serialized = ServerMessage::encode(&msg).unwrap();
        let deserialized = ServerMessage::decode(serialized.as_slice()).unwrap();
        assert_eq!(msg, deserialized);

        let msg = ServerMessage::UninstallModule {
            module_name: "test_module".into(),
        };
        let serialized = ServerMessage::encode(&msg).unwrap();
        let deserialized = ServerMessage::decode(serialized.as_slice()).unwrap();
        assert_eq!(msg, deserialized);

        let msg = ServerMessage::TaskAssignment {
            task_id: 1000,
            module_name: "test_module".into(),
            function_name: "hello_world".into(),
            inputs: vec![
                Value::new(WasmValue::I32(111)),
                Value::new(WasmValue::F32(0.123)),
                Value::new(WasmValue::Void),
            ],
        };
        let serialized = ServerMessage::encode(&msg).unwrap();
        let deserialized = ServerMessage::decode(serialized.as_slice()).unwrap();
        assert_eq!(msg, deserialized);
    }

    #[test]
    fn test_client_message_serde() {
        let msg = ClientMessage::TaskStatusUpdate {
            task_id: 1000,
            status: TaskStatus::Pending,
        };
        let serialized = ClientMessage::encode(&msg).unwrap();
        let deserialized = ClientMessage::decode(serialized.as_slice()).unwrap();
        assert_eq!(msg, deserialized);

        let msg = ClientMessage::TaskResult {
            task_id: 1000,
            result: Value::new(WasmValue::I64(1234567890)),
        };
        let serialized = ClientMessage::encode(&msg).unwrap();
        let deserialized = ClientMessage::decode(serialized.as_slice()).unwrap();
        assert_eq!(msg, deserialized);
    }
}
