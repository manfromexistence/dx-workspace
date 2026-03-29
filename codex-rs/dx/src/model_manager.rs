//! Model management system for local LLM models
//!
//! Handles model discovery, download, and path resolution with proper
//! fallback to user data directories.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Default model configuration
pub struct ModelConfig {
	pub name: String,
	pub filename: String,
	pub download_url: String,
	pub size_mb: u64,
}

impl ModelConfig {
	/// Qwen 3.5 0.8B Q4_K_M quantized model (small, fast, good quality)
	pub fn qwen_3_5_0_8b() -> Self {
		Self {
			name: "Qwen-3.5-0.8B-Q4_K_M".to_string(),
			filename: "Qwen3.5-0.8B-Q4_K_M.gguf".to_string(),
			// Direct download link from HuggingFace
			download_url: "https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF/resolve/main/qwen2.5-0.5b-instruct-q4_k_m.gguf".to_string(),
			size_mb: 380,
		}
	}
}

/// Model manager handles model path resolution and downloads
pub struct ModelManager {
	config: ModelConfig,
}

impl ModelManager {
	pub fn new(config: ModelConfig) -> Self {
		Self { config }
	}

	/// Get the model path, checking multiple locations in order:
	/// 1. User-specified path (if provided)
	/// 2. DX data directory (C:\Users\<username>\.dx\models on Windows, ~/.dx/models on Unix)
	pub fn resolve_model_path(&self, user_path: Option<&str>) -> Result<PathBuf> {
		// 1. Check user-specified path
		if let Some(path) = user_path {
			let path = PathBuf::from(path);
			if path.exists() {
				return Ok(path);
			}
		}

		// 2. Check DX data directory
		let dx_model_path = self.get_dx_models_dir()?.join(&self.config.filename);
		if dx_model_path.exists() {
			return Ok(dx_model_path);
		}

		// Model not found - return the DX data directory path where it should be downloaded
		Ok(dx_model_path)
	}

	/// Get the DX models directory path
	/// Following industry standard (like Ollama):
	/// Windows: C:\Users\<username>\.dx\models
	/// Unix: ~/.dx/models or $XDG_DATA_HOME/dx/models
	pub fn get_dx_models_dir(&self) -> Result<PathBuf> {
		let base_dir = if cfg!(target_os = "windows") {
			// Windows: C:\Users\<username>\.dx\models (similar to Ollama's .ollama)
			dirs::home_dir().context("Failed to get home directory")?.join(".dx").join("models")
		} else {
			// Unix: ~/.dx/models or $XDG_DATA_HOME/dx/models
			if let Ok(xdg_data) = std::env::var("XDG_DATA_HOME") {
				PathBuf::from(xdg_data).join("dx").join("models")
			} else {
				dirs::home_dir().context("Failed to get home directory")?.join(".dx").join("models")
			}
		};

		// Ensure directory exists
		if !base_dir.exists() {
			std::fs::create_dir_all(&base_dir)
				.context(format!("Failed to create models directory: {}", base_dir.display()))?;
		}

		Ok(base_dir)
	}

	/// Check if model exists at the resolved path
	pub fn model_exists(&self, user_path: Option<&str>) -> bool {
		self.resolve_model_path(user_path).ok().map(|p| p.exists()).unwrap_or(false)
	}

	/// Download model to the DX models directory
	pub async fn download_model<F>(&self, progress_callback: F) -> Result<PathBuf>
	where
		F: Fn(u64, u64) + Send + 'static,
	{
		let models_dir = self.get_dx_models_dir()?;
		let target_path = models_dir.join(&self.config.filename);

		// Check if already exists
		if target_path.exists() {
			return Ok(target_path);
		}

		// Download with progress
		progress_callback(0, self.config.size_mb * 1024 * 1024);

		// Use reqwest for async download
		let client = reqwest::Client::builder()
			.timeout(std::time::Duration::from_secs(3600)) // 1 hour timeout
			.build()
			.context("Failed to create HTTP client")?;

		let response =
			client.get(&self.config.download_url).send().await.context("Failed to start download")?;

		if !response.status().is_success() {
			anyhow::bail!("Download failed with status: {}", response.status());
		}

		let total_size = response.content_length().unwrap_or(self.config.size_mb * 1024 * 1024);

		// Download to temporary file first
		let temp_path = target_path.with_extension("gguf.tmp");
		let mut file =
			tokio::fs::File::create(&temp_path).await.context("Failed to create temporary file")?;

		let mut downloaded: u64 = 0;
		let mut stream = response.bytes_stream();

		use futures::StreamExt;
		use tokio::io::AsyncWriteExt;

		while let Some(chunk) = stream.next().await {
			let chunk = chunk.context("Failed to read chunk")?;
			file.write_all(&chunk).await.context("Failed to write chunk")?;
			downloaded += chunk.len() as u64;
			progress_callback(downloaded, total_size);
		}

		file.flush().await.context("Failed to flush file")?;
		drop(file);

		// Rename temp file to final name
		tokio::fs::rename(&temp_path, &target_path)
			.await
			.context("Failed to rename downloaded file")?;

		Ok(target_path)
	}

	/// Get model info for display
	pub fn get_info(&self) -> String {
		format!("{} ({} MB)", self.config.name, self.config.size_mb)
	}

	/// Get the model filename
	pub fn get_filename(&self) -> &str {
		&self.config.filename
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_dx_models_dir() {
		let config = ModelConfig::qwen_3_5_0_8b();
		let manager = ModelManager::new(config);
		let dir = manager.get_dx_models_dir().unwrap();

		// Should be in user's home directory under .dx/models (like Ollama)
		let path_str = dir.to_string_lossy();
		assert!(path_str.contains(".dx"));
		assert!(path_str.contains("models"));
	}

	#[test]
	fn test_model_config() {
		let config = ModelConfig::qwen_3_5_0_8b();
		assert_eq!(config.filename, "Qwen3.5-0.8B-Q4_K_M.gguf");
		assert!(config.size_mb > 0);
	}
}
