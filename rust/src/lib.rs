//! nullapt Rust SDK — helpers for building WASM skills.
//!
//! # Example
//!
//! ```rust,no_run
//! use nullapt_sdk::prelude::*;
//!
//! #[derive(Deserialize)]
//! struct Input { query: String }
//!
//! #[derive(Serialize)]
//! struct Output { result: String }
//!
//! #[skill_fn]
//! pub fn my_tool(input: Json<Input>) -> SkillResult<Json<Output>> {
//!     Ok(Json(Output { result: format!("got: {}", input.query) }))
//! }
//! ```

pub mod prelude {
    pub use extism_pdk::*;
    pub use serde::{Deserialize, Serialize};
    pub use crate::error::SkillError;
    pub use crate::types::SkillResult;
}

pub mod error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum SkillError {
        #[error("invalid input: {0}")]
        InvalidInput(String),
        #[error("network error: {0}")]
        Network(String),
        #[error("permission denied: {0}")]
        PermissionDenied(String),
        #[error("{0}")]
        Other(String),
    }
}

pub mod types {
    use crate::error::SkillError;
    pub type SkillResult<T> = Result<T, SkillError>;
}

pub mod manifest {
    use serde::{Deserialize, Serialize};

    /// Mirrors the SKILL.json schema for programmatic generation.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SkillManifest {
        pub schema_version: String,
        pub name: String,
        pub version: String,
        pub description: String,
        pub author: String,
        pub homepage: String,
        pub license: String,
        pub permissions: Permissions,
        pub entry: String,
        pub interface: Interface,
        pub signature: Signature,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct Permissions {
        pub network: NetworkPerms,
        pub filesystem: FsPerms,
        #[serde(default)]
        pub env: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct NetworkPerms {
        pub allowed: bool,
        #[serde(default)]
        pub domains: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct FsPerms {
        #[serde(default)]
        pub read: Vec<String>,
        #[serde(default)]
        pub write: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct Interface {
        pub tools: Vec<Tool>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Tool {
        pub name: String,
        pub description: String,
        pub input_schema: serde_json::Value,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct Signature {
        pub algorithm: String,
        pub public_key: String,
        pub value: String,
    }
}
