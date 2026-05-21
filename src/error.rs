//! SDK 공통 에러.

use thiserror::Error;

use crate::Permissions;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("permission denied: requires {required:?}, granted {granted:?}")]
pub struct PermissionDenied {
    pub required: Permissions,
    pub granted: Permissions,
}

#[derive(Debug, Error)]
pub enum ToolError {
    #[error(transparent)]
    PermissionDenied(#[from] PermissionDenied),

    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error("internal: {0}")]
    Internal(String),
}

impl ToolError {
    /// JSON-RPC error code 매핑 (transport adapter가 호출).
    /// - PermissionDenied → -32001 (server-defined)
    /// - InvalidArgument → -32602 (Invalid params)
    /// - Internal → -32603 (Internal error)
    pub fn json_rpc_code(&self) -> i32 {
        match self {
            ToolError::PermissionDenied(_) => -32001,
            ToolError::InvalidArgument(_) => -32602,
            ToolError::Internal(_) => -32603,
        }
    }
}
