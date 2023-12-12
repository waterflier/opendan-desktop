use serde::{Deserialize, Serialize, Serializer};

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcRequest {
    function_name: String,
    params: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcResponse {
    pub result: serde_json::Value,
    pub error: Option<String>,
    pub code: ErrorCode,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum ErrorCode {
    Ok = 0,
    InternalError = 10001,
    CommandError = 10010,
}
// 实现Serialize trait，将枚举转换为数字
impl Serialize for ErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32((*self) as u32)
    }
}

macro_rules! success_response {
    ($msg:expr) => {{
        let mut resp = tide::Response::new(tide::StatusCode::Ok);
        let rpc_resp = crate::api::def::RpcResponse {
            result: tide::prelude::json!($msg),
            error: None,
            code: crate::api::def::ErrorCode::Ok,
        };
        let json_string = serde_json::to_string(&rpc_resp).unwrap();
        resp.set_body(json_string);
        return Ok(resp);
    }};
}

macro_rules! error_response {
    ($e:expr, $code:expr) => {{
        let mut resp = tide::Response::new(tide::StatusCode::Ok);
        let rpc_resp = crate::api::def::RpcResponse {
            result: tide::prelude::json!(""),
            error: Some($e.to_string()),
            code: $code,
        };
        let json_string = serde_json::to_string(&rpc_resp).unwrap();
        resp.set_body(json_string);
        return Ok(resp);
    }};
}
