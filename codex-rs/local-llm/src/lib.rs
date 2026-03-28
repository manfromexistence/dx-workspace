//! Local LLM integration using llama.cpp
//!
//! This module provides direct integration with llama.cpp for running
//! language models locally without requiring external servers.
//!
//! NOTE: Currently disabled due to C++ runtime symbol conflicts with V8.
//! The llama-cpp-2 dependency is commented out in Cargo.toml.

use anyhow::Result;
use std::path::PathBuf;

/// Local LLM client (stub implementation - llama.cpp disabled)
#[derive(Clone)]
pub struct LocalLlm;

impl LocalLlm {
    /// Create a new LocalLlm instance
    pub fn new() -> Self {
        Self
    }

    /// Create a LocalLlm with a custom model path
    pub fn with_model_path(_model_path: PathBuf) -> Self {
        Self
    }

    /// Initialize the model and backend (stub)
    pub async fn initialize(&self) -> Result<()> {
        Err(anyhow::anyhow!(
            "Local LLM is disabled due to C++ runtime conflicts with V8"
        ))
    }

    /// Generate a response to the given prompt (stub)
    pub async fn generate(&self, _prompt: &str) -> Result<String> {
        Err(anyhow::anyhow!(
            "Local LLM is disabled due to C++ runtime conflicts with V8"
        ))
    }

    /// Generate a response with streaming output (stub)
    pub async fn generate_stream<F>(&self, _prompt: &str, _callback: F) -> Result<()>
    where
        F: Fn(String) + Send + 'static,
    {
        Err(anyhow::anyhow!(
            "Local LLM is disabled due to C++ runtime conflicts with V8"
        ))
    }

    /// Check if the model is initialized
    pub fn is_initialized(&self) -> bool {
        false
    }

    /// Get the model name for display
    pub fn get_model_name(&self) -> String {
        "Local LLM (disabled)".to_string()
    }

    /// Get the context window size
    pub fn context_window_size(&self) -> u32 {
        0
    }

    /// Get the current model path
    pub fn get_model_path(&self) -> Result<PathBuf> {
        Err(anyhow::anyhow!("Local LLM is disabled"))
    }

    /// Clear conversation history
    pub fn clear_history(&self) -> Result<()> {
        Ok(())
    }
}

impl Default for LocalLlm {
    fn default() -> Self {
        Self::new()
    }
}
