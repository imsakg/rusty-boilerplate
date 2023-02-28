//! Reusable building blocks for safe Rust project scaffolding.
//!
//! Inspect a project before deciding where to write it:
//!
//! ```
//! use rusty_boilerplate::plan::Plan;
//! let plan = Plan::project("hello", "lib")?;
//! assert!(plan.files.contains_key(std::path::Path::new("src/lib.rs")));
//! # Ok::<(), String>(())
//! ```
pub mod app;
pub mod cli;
pub mod config;
pub mod generate;
pub mod local;
pub mod name;
pub mod plan;
pub mod render;
pub mod template;
