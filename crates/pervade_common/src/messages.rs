use serde::{Deserialize, Serialize};

use crate::value::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    ModuleTransfer {
        module_name: String,
        module_data: Vec<u8>,
    },
    TaskAssignment {
        module_name: String,
        function_name: String,
        inputs: Vec<Value>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    TaskStatusUpdate {
        task_id: u64,
        status: TaskStatus,
    },
    TaskResult {
        task_id: u64,
        result: Value,
    },
    Error {
        message: String,
    },
}

#[cfg(test)]
mod tests {
    use wamr_rust_sdk::value::WasmValue;

    use crate::value::Value;
    use super::*;

    #[test]
    fn test_server_message_serde() {
        let msg = ServerMessage::ModuleTransfer {
            module_name: "test_module".into(),
            module_data: vec![1, 2, 3, 4],
        };
        let serialized = serde_json::to_string(&msg).unwrap();
        let deserialized: ServerMessage = serde_json::from_str(&serialized).unwrap();
        assert_eq!(msg, deserialized);

        let msg = ServerMessage::TaskAssignment {
            module_name: "test_module".into(),
            function_name: "hello_world".into(),
            inputs: vec![
                Value::new(WasmValue::I32(111)),
                Value::new(WasmValue::F32(0.123)),
                Value::new(WasmValue::Void),
            ],
        };
        let serialized = serde_json::to_string(&msg).unwrap();
        let deserialized: ServerMessage = serde_json::from_str(&serialized).unwrap();
        assert_eq!(msg, deserialized);
    }

    #[test]
    fn test_client_message_serde() {
        let msg = ClientMessage::TaskResult {
            task_id: 999,
            result: Value::new(WasmValue::I64(1234567890)),
        };
        let serialized = serde_json::to_string(&msg).unwrap();
        let deserialized: ClientMessage = serde_json::from_str(&serialized).unwrap();
        assert_eq!(msg, deserialized);

        let msg = ClientMessage::Error {
            message: "Something went wrong".into(),
        };
        let serialized = serde_json::to_string(&msg).unwrap();
        let deserialized: ClientMessage = serde_json::from_str(&serialized).unwrap();
        assert_eq!(msg, deserialized);
    }
}
