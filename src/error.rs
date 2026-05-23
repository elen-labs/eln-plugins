//! SDK 공통 에러.

use serde_json::{Value, json};
use thiserror::Error;

use crate::Permissions;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("permission denied: requires {required:?}, granted {granted:?}")]
#[non_exhaustive]
pub struct PermissionDenied {
    pub required: Permissions,
    pub granted: Permissions,
}

impl PermissionDenied {
    pub fn new(required: Permissions, granted: Permissions) -> Self {
        Self { required, granted }
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ToolError {
    #[error(transparent)]
    PermissionDenied(#[from] PermissionDenied),

    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error("internal: {0}")]
    Internal(String),
}

/// PermissionDenied용 JSON-RPC error code.
///
/// JSON-RPC `server-defined` 영역 (-32000 ~ -32099). 현 rmcp 1.5.0 확장
/// (`RESOURCE_NOT_FOUND=-32002`, `URL_ELICITATION_REQUIRED=-32042`)과 겹치지 않음.
/// 미래에 MCP/rmcp가 `-32001`을 예약할 수 있으므로 transport adapter는 코드만
/// 보지 말고 `data.kind` 식별자도 함께 확인할 것 (codex review 1 권고).
pub const ERROR_CODE_PERMISSION_DENIED: i32 = -32001;

impl ToolError {
    /// JSON-RPC error code 매핑 (transport adapter가 호출).
    pub fn json_rpc_code(&self) -> i32 {
        match self {
            ToolError::PermissionDenied(_) => ERROR_CODE_PERMISSION_DENIED,
            ToolError::InvalidArgument(_) => -32602,
            ToolError::Internal(_) => -32603,
        }
    }

    /// JSON-RPC `error.data` payload — kind 식별자 + 세부 필드.
    ///
    /// transport adapter가 error response 직렬화 시 첨부. `data.kind`로 코드 충돌
    /// 가능성과 무관하게 SDK-level error를 식별 가능 (codex review 1 권고).
    pub fn json_rpc_data(&self) -> Value {
        match self {
            ToolError::PermissionDenied(pd) => json!({
                "kind": "permission_denied",
                "required": pd.required.bits(),
                "granted": pd.granted.bits(),
            }),
            ToolError::InvalidArgument(msg) => json!({
                "kind": "invalid_argument",
                "message": msg,
            }),
            ToolError::Internal(msg) => json!({
                "kind": "internal",
                "message": msg,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permission_denied_json_rpc_data_carries_kind_and_bits() {
        let err = ToolError::PermissionDenied(PermissionDenied {
            required: Permissions::WRITE,
            granted: Permissions::READ,
        });
        assert_eq!(err.json_rpc_code(), ERROR_CODE_PERMISSION_DENIED);
        let data = err.json_rpc_data();
        assert_eq!(data["kind"], "permission_denied");
        assert_eq!(data["required"], Permissions::WRITE.bits());
        assert_eq!(data["granted"], Permissions::READ.bits());
    }
}
