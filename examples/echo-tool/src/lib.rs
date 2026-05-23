//! # echo-tool — in-process SDK example
//!
//! READ-only `echo(msg)` 1개. `eln-plugin-sdk`의 `ToolHandler` / `ToolDescriptor`
//! / `PluginServer` 표면이 외부 소비자에서 자연스럽게 정합하는지 확인하는 첫 번째
//! 도그푸드 사례.
//!
//! subprocess + IPC plugin loader는 S3 이후 — 본 crate는 trait shape + descriptor
//! 표면 검증용. 권한 거절 path 검증은 `tiny-write` 쪽에서 다룬다.

use async_trait::async_trait;
use eln_plugin_sdk::{
    CallContext, Permissions, PluginServer, ToolDescriptor, ToolError, ToolHandler,
};
use serde_json::{Value, json};

pub struct EchoTool;

#[async_trait]
impl ToolHandler for EchoTool {
    fn name(&self) -> &str {
        "echo"
    }

    fn description(&self) -> &str {
        "Echo the provided `msg` string back unchanged."
    }

    async fn call(&self, _ctx: &CallContext, args: Value) -> Result<Value, ToolError> {
        let msg = args
            .get("msg")
            .and_then(Value::as_str)
            .ok_or_else(|| ToolError::InvalidArgument("missing `msg` (string)".into()))?;
        Ok(json!({ "echoed": msg }))
    }
}

pub struct EchoServer;

impl PluginServer for EchoServer {
    fn tools(&self) -> Vec<ToolDescriptor> {
        vec![ToolDescriptor::new(EchoTool)
            .with_required_permissions(Permissions::READ)
            .with_input_schema(json!({
                "type": "object",
                "properties": { "msg": { "type": "string" } },
                "required": ["msg"]
            }))]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eln_plugin_sdk::Identity;

    fn ctx_read() -> CallContext {
        CallContext::new("test-sess".into(), Identity::Human, Permissions::READ)
    }

    #[tokio::test]
    async fn echo_returns_msg() {
        let tool = EchoTool;
        let out = tool
            .call(&ctx_read(), json!({ "msg": "hi" }))
            .await
            .unwrap();
        assert_eq!(out["echoed"], "hi");
    }

    #[tokio::test]
    async fn echo_rejects_missing_msg() {
        let tool = EchoTool;
        let err = tool.call(&ctx_read(), json!({})).await.unwrap_err();
        assert!(matches!(err, ToolError::InvalidArgument(_)));
    }

    #[test]
    fn server_lists_one_tool_with_read() {
        let server = EchoServer;
        let tools = server.tools();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name(), "echo");
        assert_eq!(tools[0].required_permissions(), Permissions::READ);
        assert!(tools[0].input_schema().is_some());
    }
}
