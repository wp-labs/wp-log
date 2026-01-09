pub mod conf;

#[macro_use]
pub mod macro_def;

// Re-export dependencies for macro use at call sites
pub mod re_exports {
    pub use log;
}
