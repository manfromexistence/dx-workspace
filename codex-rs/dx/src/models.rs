// Model management for DX
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelInfo {
	pub id: String,
	pub display_name: String,
	pub description: String,
	pub provider: ModelProvider,
	pub is_default: bool,
	pub is_unlimited: bool,
	pub context_window: Option<u32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ModelProvider {
	Local,
	Codex,
}

impl ModelProvider {
	pub fn name(&self) -> &'static str {
		match self {
			Self::Local => "Local",
			Self::Codex => "Codex",
		}
	}
}

/// Get all available models
pub fn get_available_models() -> Vec<ModelInfo> {
	vec![
		// Local model (default, unlimited)
		ModelInfo {
			id: "local-infinity".to_string(),
			display_name: "Infinity".to_string(),
			description: "Unlimited local model with infinite context".to_string(),
			provider: ModelProvider::Local,
			is_default: true,
			is_unlimited: true,
			context_window: None,
		},
		// Codex models (from models.json)
		ModelInfo {
			id: "gpt-5.4".to_string(),
			display_name: "GPT-5.4".to_string(),
			description: "Latest frontier agentic coding model".to_string(),
			provider: ModelProvider::Codex,
			is_default: false,
			is_unlimited: false,
			context_window: Some(272_000),
		},
		ModelInfo {
			id: "mistral-small-latest".to_string(),
			display_name: "Mistral Small".to_string(),
			description: "Mistral's fastest and most cost-effective model".to_string(),
			provider: ModelProvider::Codex,
			is_default: false,
			is_unlimited: false,
			context_window: Some(128_000),
		},
		ModelInfo {
			id: "gpt-5.3-codex".to_string(),
			display_name: "GPT-5.3 Codex".to_string(),
			description: "Powerful coding model with extended context".to_string(),
			provider: ModelProvider::Codex,
			is_default: false,
			is_unlimited: false,
			context_window: Some(200_000),
		},
		ModelInfo {
			id: "gpt-5.1-codex-mini".to_string(),
			display_name: "GPT-5.1 Codex Mini".to_string(),
			description: "Fast and efficient coding model".to_string(),
			provider: ModelProvider::Codex,
			is_default: false,
			is_unlimited: false,
			context_window: Some(128_000),
		},
		ModelInfo {
			id: "claude-3.5-sonnet".to_string(),
			display_name: "Claude 3.5 Sonnet".to_string(),
			description: "Anthropic's powerful coding assistant".to_string(),
			provider: ModelProvider::Codex,
			is_default: false,
			is_unlimited: false,
			context_window: Some(200_000),
		},
	]
}

/// Get the default model
pub fn get_default_model() -> ModelInfo {
	get_available_models().into_iter().find(|m| m.is_default).expect("No default model found")
}

/// Get a model by ID
pub fn get_model_by_id(id: &str) -> Option<ModelInfo> {
	get_available_models().into_iter().find(|m| m.id == id)
}

/// Get models by provider
pub fn get_models_by_provider(provider: ModelProvider) -> Vec<ModelInfo> {
	get_available_models().into_iter().filter(|m| m.provider == provider).collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_default_model_is_local() {
		let default = get_default_model();
		assert_eq!(default.provider, ModelProvider::Local);
		assert!(default.is_unlimited);
		assert!(default.is_default);
	}

	#[test]
	fn test_get_model_by_id() {
		let model = get_model_by_id("gpt-5.4");
		assert!(model.is_some());
		assert_eq!(model.unwrap().display_name, "GPT-5.4");
	}

	#[test]
	fn test_get_models_by_provider() {
		let local_models = get_models_by_provider(ModelProvider::Local);
		assert_eq!(local_models.len(), 1);

		let codex_models = get_models_by_provider(ModelProvider::Codex);
		assert!(codex_models.len() > 1);
	}
}
