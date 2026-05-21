//! Plugin server 표면 — tool 집합 노출.

use std::sync::Arc;

use crate::ToolHandler;

/// Tool descriptor — name + description + handler.
#[derive(Clone)]
pub struct ToolDescriptor {
    pub handler: Arc<dyn ToolHandler>,
}

impl ToolDescriptor {
    pub fn new<H: ToolHandler + 'static>(handler: H) -> Self {
        Self {
            handler: Arc::new(handler),
        }
    }

    pub fn name(&self) -> &str {
        self.handler.name()
    }

    pub fn description(&self) -> &str {
        self.handler.description()
    }
}

pub trait PluginServer {
    fn tools(&self) -> Vec<ToolDescriptor>;
}
