//! # tiny-write — in-process SDK example with WRITE permission gate
//!
//! tool 2개:
//! - `read_note(id)` — READ
//! - `append_note(id, text)` — WRITE 필요. `ctx.permissions`에 `WRITE`가 없으면
//!   `ToolError::PermissionDenied`로 떨어짐.
//!
//! S2.3 (b) Permissions enforcement PoC와 정합. transport-level grant derivation
//! (ApiKey → Permissions)은 S3 scope. subprocess + IPC plugin loader도 S3 이후 —
//! 본 crate는 권한 거절 path가 SDK 표면에서 자기일치하는지 검증하는 자리.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use eln_plugin_sdk::{
    CallContext, PermissionDenied, Permissions, PluginServer, ToolDescriptor, ToolError,
    ToolHandler,
};
use serde_json::{Value, json};

#[derive(Default)]
pub struct NoteStore {
    notes: Mutex<HashMap<String, String>>,
}

impl NoteStore {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct ReadNote {
    store: Arc<NoteStore>,
}

impl ReadNote {
    pub fn new(store: Arc<NoteStore>) -> Self {
        Self { store }
    }
}

#[async_trait]
impl ToolHandler for ReadNote {
    fn name(&self) -> &str {
        "read_note"
    }

    fn description(&self) -> &str {
        "Return the current contents of note `id` (empty if missing)."
    }

    async fn call(&self, _ctx: &CallContext, args: Value) -> Result<Value, ToolError> {
        let id = args
            .get("id")
            .and_then(Value::as_str)
            .ok_or_else(|| ToolError::InvalidArgument("missing `id` (string)".into()))?;
        let map = self
            .store
            .notes
            .lock()
            .map_err(|e| ToolError::Internal(format!("note store poisoned: {e}")))?;
        Ok(json!({
            "id": id,
            "text": map.get(id).cloned().unwrap_or_default(),
        }))
    }
}

pub struct AppendNote {
    store: Arc<NoteStore>,
}

impl AppendNote {
    pub fn new(store: Arc<NoteStore>) -> Self {
        Self { store }
    }
}

#[async_trait]
impl ToolHandler for AppendNote {
    fn name(&self) -> &str {
        "append_note"
    }

    fn description(&self) -> &str {
        "Append `text` to note `id`. Requires WRITE permission."
    }

    async fn call(&self, ctx: &CallContext, args: Value) -> Result<Value, ToolError> {
        if !ctx.permissions.contains(Permissions::WRITE) {
            return Err(PermissionDenied {
                required: Permissions::WRITE,
                granted: ctx.permissions,
            }
            .into());
        }
        let id = args
            .get("id")
            .and_then(Value::as_str)
            .ok_or_else(|| ToolError::InvalidArgument("missing `id` (string)".into()))?;
        let text = args
            .get("text")
            .and_then(Value::as_str)
            .ok_or_else(|| ToolError::InvalidArgument("missing `text` (string)".into()))?;
        let mut map = self
            .store
            .notes
            .lock()
            .map_err(|e| ToolError::Internal(format!("note store poisoned: {e}")))?;
        let entry = map.entry(id.to_string()).or_default();
        entry.push_str(text);
        let len = entry.len();
        Ok(json!({ "id": id, "len": len }))
    }
}

pub struct TinyWriteServer {
    store: Arc<NoteStore>,
}

impl TinyWriteServer {
    pub fn new() -> Self {
        Self {
            store: Arc::new(NoteStore::new()),
        }
    }

    pub fn store(&self) -> Arc<NoteStore> {
        self.store.clone()
    }
}

impl Default for TinyWriteServer {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginServer for TinyWriteServer {
    fn tools(&self) -> Vec<ToolDescriptor> {
        vec![
            ToolDescriptor::new(ReadNote::new(self.store.clone()))
                .with_required_permissions(Permissions::READ)
                .with_input_schema(json!({
                    "type": "object",
                    "properties": { "id": { "type": "string" } },
                    "required": ["id"]
                })),
            ToolDescriptor::new(AppendNote::new(self.store.clone()))
                .with_required_permissions(Permissions::WRITE)
                .with_input_schema(json!({
                    "type": "object",
                    "properties": {
                        "id":   { "type": "string" },
                        "text": { "type": "string" }
                    },
                    "required": ["id", "text"]
                })),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eln_plugin_sdk::Identity;

    fn ctx(perms: Permissions) -> CallContext {
        CallContext {
            session_id: "test-sess".into(),
            identity: Identity::Human,
            permissions: perms,
        }
    }

    #[tokio::test]
    async fn read_note_returns_empty_for_missing() {
        let tool = ReadNote::new(Arc::new(NoteStore::new()));
        let out = tool
            .call(&ctx(Permissions::READ), json!({ "id": "n1" }))
            .await
            .unwrap();
        assert_eq!(out["text"], "");
        assert_eq!(out["id"], "n1");
    }

    #[tokio::test]
    async fn append_note_rejects_read_only_ctx() {
        let tool = AppendNote::new(Arc::new(NoteStore::new()));
        let err = tool
            .call(
                &ctx(Permissions::READ),
                json!({ "id": "n1", "text": "hello" }),
            )
            .await
            .unwrap_err();
        match err {
            ToolError::PermissionDenied(pd) => {
                assert_eq!(pd.required, Permissions::WRITE);
                assert_eq!(pd.granted, Permissions::READ);
            }
            other => panic!("expected PermissionDenied, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn append_note_persists_with_write() {
        let store = Arc::new(NoteStore::new());
        let appender = AppendNote::new(store.clone());
        appender
            .call(
                &ctx(Permissions::WRITE),
                json!({ "id": "n1", "text": "abc" }),
            )
            .await
            .unwrap();
        let reader = ReadNote::new(store);
        let out = reader
            .call(&ctx(Permissions::READ), json!({ "id": "n1" }))
            .await
            .unwrap();
        assert_eq!(out["text"], "abc");
    }

    #[tokio::test]
    async fn append_note_admin_also_succeeds() {
        let tool = AppendNote::new(Arc::new(NoteStore::new()));
        let out = tool
            .call(
                &ctx(Permissions::ADMIN),
                json!({ "id": "n2", "text": "x" }),
            )
            .await
            .unwrap();
        assert_eq!(out["len"], 1);
    }

    #[test]
    fn server_lists_two_tools_with_distinct_perms() {
        let server = TinyWriteServer::new();
        let tools = server.tools();
        assert_eq!(tools.len(), 2);
        let by_name: HashMap<_, _> = tools
            .iter()
            .map(|t| (t.name().to_string(), t.required_permissions()))
            .collect();
        assert_eq!(by_name.get("read_note"), Some(&Permissions::READ));
        assert_eq!(by_name.get("append_note"), Some(&Permissions::WRITE));
    }
}
