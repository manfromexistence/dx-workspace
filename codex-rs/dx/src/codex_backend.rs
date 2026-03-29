// Codex backend integration (no UI components)
// This module initializes and manages the Codex backend for AI processing

use anyhow::Result;
use codex_core::models_manager::collaboration_mode_presets::CollaborationModesConfig;
use codex_core::{AuthManager, Config, ThreadManager};
use codex_otel::SessionTelemetry;
use codex_protocol::ThreadId;
use codex_protocol::protocol::{Op, SessionSource};
use codex_utils_absolute_path::AbsolutePathBuf;
use std::sync::Arc;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

pub struct CodexBackend {
	pub thread_manager: Arc<ThreadManager>,
	pub auth_manager: Arc<AuthManager>,
	pub config: Config,
	pub op_tx: UnboundedSender<Op>,
	pub op_rx: UnboundedReceiver<Op>,
}

/// Initialize Codex backend with Mistral as default provider
pub async fn initialize_codex_backend() -> Result<CodexBackend> {
	// 1. Find Codex home
	let codex_home = codex_core::config::find_codex_home()?.to_path_buf();
	let cwd = AbsolutePathBuf::current_dir()?;

	// 2. Load config with Mistral as default
	let cli_kv_overrides = vec![
		// Set Mistral as the default provider
		"model_provider_id=mistral".to_string(),
		"model=mistral-large-latest".to_string(),
	];

	let config_toml = codex_core::config::load_config_as_toml_with_cli_overrides(
		&codex_home,
		&cwd,
		cli_kv_overrides.clone(),
	)
	.await?;

	// 3. Create AuthManager
	let auth_manager = AuthManager::shared(
		codex_home.clone(),
		false,
		config_toml.cli_auth_credentials_store.unwrap_or_default(),
	);

	// 4. Create cloud requirements
	let chatgpt_base_url = config_toml
		.chatgpt_base_url
		.clone()
		.unwrap_or_else(|| "https://chatgpt.com/backend-api/".to_string());

	let cloud_requirements = codex_cloud_requirements::cloud_requirements_loader(
		auth_manager.clone(),
		chatgpt_base_url,
		codex_home.clone(),
	);

	// 5. Build config
	let mut config = codex_core::config::ConfigBuilder::default()
		.codex_home(codex_home.clone())
		.cli_overrides(cli_kv_overrides)
		.cloud_requirements(cloud_requirements.clone())
		.loader_overrides(codex_core::config_loader::LoaderOverrides::default())
		.build()
		.await?;

	// 6. Ensure Mistral is set as the model
	config.model = Some("mistral-large-latest".to_string());
	config.model_provider_id = "mistral".to_string();

	// 7. Get auth for session telemetry
	let auth = auth_manager.auth().await;
	let auth_ref = auth.as_ref();

	// 8. Create session telemetry
	let thread_id = ThreadId::new();
	let session_telemetry = SessionTelemetry::new(
		thread_id,
		"mistral-large-latest",
		"mistral-large-latest",
		auth_ref.and_then(codex_core::CodexAuth::get_account_id),
		auth_ref.and_then(codex_core::CodexAuth::get_account_email),
		auth_ref.map(codex_core::CodexAuth::auth_mode).map(|m| m.into()),
		codex_core::default_client::originator().value,
		config.otel.log_user_prompt,
		"dx-tui/1.0".to_string(),
		SessionSource::Cli,
	);

	// 9. Create ThreadManager
	let thread_manager = Arc::new(ThreadManager::new(
		&config,
		auth_manager.clone(),
		SessionSource::Cli,
		CollaborationModesConfig { default_mode_request_user_input: false },
	));

	// 10. Create Op channels
	let (op_tx, op_rx) = mpsc::unbounded_channel();

	Ok(CodexBackend { thread_manager, auth_manager, config, op_tx, op_rx })
}
