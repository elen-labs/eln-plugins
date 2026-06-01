//! Tool 호출 표면 — plugin이 노출하는 단위 tool의 trait.

use async_trait::async_trait;
use serde_json::Value;

use crate::{Identity, Permissions, ToolError};

/// Tool 호출 시 plugin handler에 전달되는 컨텍스트.
///
/// `session_id`는 transport가 발급 (stdio: UUID v4, HTTP: `Mcp-Session-Id`).
/// `permissions`/`identity`는 S3부터 ApiKey-derived — 활성 keystore HTTP에서는
/// Bearer 키 레코드로 유도, stdio/익명은 transport default(ADMIN/READ) + Human.
/// `key_id`는 그 인증 키의 식별자(발급 키 레코드 id) — audit 추적·키별 동작 근거.
/// stdio·익명·인증 실패 경로에서는 `None`.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct CallContext {
    pub session_id: String,
    pub identity: Identity,
    pub permissions: Permissions,
    /// 이 호출을 인증한 API key 레코드의 id (`k_…`). 키 인증이 아니면 `None`.
    pub key_id: Option<String>,
}

impl CallContext {
    pub fn new(session_id: String, identity: Identity, permissions: Permissions) -> Self {
        Self {
            session_id,
            identity,
            permissions,
            key_id: None,
        }
    }

    /// 인증 키 id를 실어 반환 (builder). `new` 호출부 무영향 — key_id 기본은 `None`.
    pub fn with_key_id(mut self, key_id: Option<String>) -> Self {
        self.key_id = key_id;
        self
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
