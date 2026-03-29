// Providers submenu - show models list first, then management options
pub fn get_submenu() -> Vec<(&'static str, &'static str)> {
	vec![
		("Back", ""),
		("1. List", "SUBMENU:models-list"),
		("2. API Key Management", ""),
		("3. Token Limits", ""),
		("4. Rate Limiting", ""),
		("5. Custom Provider", ""),
	]
}

// Models list submenu (shown when "List" is selected)
pub fn get_models_submenu() -> Vec<(&'static str, &'static str)> {
	vec![
		("Back", ""),
		("Infinity", "MODEL:infinity|Local"),
		("GPT-5.4 (272k)", "MODEL:gpt-5.4|OpenAI"),
		("Mistral Small (32k)", "MODEL:mistral-small|Mistral"),
		("GPT-5.3 Codex (272k)", "MODEL:gpt-5.3-codex|OpenAI"),
		("GPT-5.1 Codex Mini (272k)", "MODEL:gpt-5.1-codex-mini|OpenAI"),
		("Claude 3.5 Sonnet (200k)", "MODEL:claude-3.5-sonnet|Anthropic"),
	]
}
