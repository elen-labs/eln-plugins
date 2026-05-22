//! Plugin server 표면 — tool 집합 노출.

use std::sync::Arc;

use serde_json::Value;

use crate::{Permissions, ToolHandler};

/// Tool descriptor — handler + metadata.
///
/// `#[non_exhaustive]`: 미래에 field 추가가 caller code를 깨지 않도록 (codex review 1
/// 권고). 외부에서는 `ToolDescriptor::new(handler).with_*` builder로만 구성.
#[derive(Clone)]
#[non_exhaustive]
pub struct ToolDescriptor {
    handler: Arc<dyn ToolHandler>,
    input_schema: Option<Value>,
    required_permissions: Permissions,
    annotations: Option<Value>,
}

impl ToolDescriptor {
    pub fn new<H: ToolHandler + 'static>(handler: H) -> Self {
        Self {
            handler: Arc::new(handler),
            input_schema: None,
            required_permissions: Permissions::empty(),
            annotations: None,
        }
    }

    pub fn with_input_schema(mut self, schema: Value) -> Self {
        self.input_schema = Some(schema);
        self
    }

    pub fn with_required_permissions(mut self, perms: Permissions) -> Self {
        self.required_permissions = perms;
        self
    }

    pub fn with_annotations(mut self, ann: Value) -> Self {
        self.annotations = Some(ann);
        self
    }

    pub fn name(&self) -> &str {
        self.handler.name()
    }

    pub fn description(&self) -> &str {
        self.handler.description()
    }

    pub fn input_schema(&self) -> Option<&Value> {
        self.input_schema.as_ref()
    }

    pub fn required_permissions(&self) -> Permissions {
        self.required_permissions
    }

    pub fn annotations(&self) -> Option<&Value> {
        self.annotations.as_ref()
    }

    /// transport adapter 전용 handler accessor.
    /// SDK consumer는 일반적으로 `name/description/...` 만 보면 충분.
    pub fn handler(&self) -> &Arc<dyn ToolHandler> {
        &self.handler
    }
}

pub trait PluginServer {
    fn tools(&self) -> Vec<ToolDescriptor>;
}
