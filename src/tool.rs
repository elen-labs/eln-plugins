//! Tool 호출 표면 — plugin이 노출하는 단위 tool의 trait.

use async_trait::async_trait;
use serde_json::Value;

use crate::{Identity, Permissions, ToolError};

/// Tool 호출 시 plugin handler에 전달되는 컨텍스트.
///
/// `session_id`는 transport가 발급 (stdio: UUID v4, HTTP: `Mcp-Session-Id`).
/// `permissions`는 caller에게 부여된 권한 비트 — S2는 stdio=ADMIN, HTTP=READ
/// hard-code, S3에서 ApiKey-derived로 교체.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct CallContext {
    pub session_id: String,
    pub identity: Identity,
    pub permissions: Permissions,
}

impl CallContext {
    pub fn new(session_id: String, identity: Identity, permissions: Permissions) -> Self {
        Self {
            session_id,
            identity,
            permissions,
        }
    }
}

#[async_trait]
pub trait ToolHandler: Send + Sync {
    /// Tool 이름 (MCP tool name과 매핑).
    fn name(&self) -> &str;

    /// Tool 한 줄 설명 (MCP description).
    fn description(&self) -> &str;

    /// 핸들러 본체.
    async fn call(&self, ctx: &CallContext, args: Value) -> Result<Value, ToolError>;
}
