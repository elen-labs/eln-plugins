//! # eln-plugin-sdk
//!
//! Plugin API platform for the [elendirna](https://github.com/elen-labs/elendirna) project.
//!
//! Defines the interface contract between elendirna core and plugin modules:
//! permissions, identity, tool call shape, server surface.
//!
//! ## Status (S2)
//!
//! Trait-shaped skeleton. rmcp dep 미포함 — transport-agnostic. SDK proc-macro
//! (`#[mcp_tool]`)는 도입하지 않음 (rmcp `#[tool]` macro와 layer 충돌 회피).
//! API key validation / signature·hash·registry / audit hook은 본 SDK 범위 밖 (S3).

pub mod api_key;
pub mod error;
pub mod identity;
pub mod permissions;
pub mod server;
pub mod tool;

pub use api_key::ApiKey;
pub use error::{PermissionDenied, ToolError};
pub use identity::Identity;
pub use permissions::Permissions;
pub use server::{PluginServer, ToolDescriptor};
pub use tool::{CallContext, ToolHandler};

/// SDK crate 버전 (`CARGO_PKG_VERSION`). 호환성 진단용 — core가 session_start 응답에
/// core 버전과 함께 노출해 plugin SDK ↔ core 버전 skew를 한눈에 드러낸다.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
