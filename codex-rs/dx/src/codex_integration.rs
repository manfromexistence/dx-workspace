// Full Codex TUI integration - uses the already-compiled Codex TUI from codex_tui_dx crate
// The entire Codex TUI is already part of DX!
// We just need to initialize ChatWidget and render it.

use codex_protocol::protocol::Op;
use codex_protocol::user_input::UserInput;
use codex_tui_dx::app_event::AppEvent;
use codex_tui_dx::app_event_sender::AppEventSender;
use codex_tui_dx::bottom_pane::FeedbackAudience;
use codex_tui_dx::chatwidget::{ChatWidget, ChatWidgetInit};
use codex_tui_dx::render::renderable::Renderable;
use codex_tui_dx::tui::FrameRequester;
use crossterm::event::KeyEvent;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub struct CodexWidgetState {
	pub chat_widget: ChatWidget,
	pub app_event_rx: UnboundedReceiver<AppEvent>,
	pub codex_op_tx: UnboundedSender<Op>,
}

impl CodexWidgetState {
	/// Process pending events from ChatWidget
	pub fn process_events(&mut self) {
		// Process all pending AppEvents
		while let Ok(event) = self.app_event_rx.try_recv() {
			match event {
				AppEvent::CodexEvent(codex_event) => {
					// Forward Codex protocol events to ChatWidget
					self.chat_widget.handle_codex_event(codex_event);
				}
				_ => {
					// Other events are not handled in this minimal integration
					// The full codex-tui-dx app handles these, but we don't need them for basic functionality
				}
			}
		}
	}
}

/// Initialize a minimal Codex ChatWidget for rendering in DX
pub async fn initialize_codex_widget() -> anyhow::Result<CodexWidgetState> {
	use codex_cloud_requirements::cloud_requirements_loader;
	use codex_core::config::{
		ConfigBuilder, find_codex_home, load_config_as_toml_with_cli_overrides,
	};
	use codex_core::config_loader::LoaderOverrides;
	use codex_core::models_manager::collaboration_mode_presets::CollaborationModesConfig;
	use codex_core::{AuthManager, ThreadManager};
	use codex_otel::SessionTelemetry;
	use codex_protocol::protocol::SessionSource;
	use codex_utils_absolute_path::AbsolutePathBuf;
	use std::sync::Arc;
	use std::sync::atomic::AtomicBool;
	use tokio::sync::{broadcast, mpsc};

	// 1. Find Codex home
	let codex_home = find_codex_home()?.to_path_buf();
	let cwd = AbsolutePathBuf::current_dir()?;

	// 2. Load config
	let cli_kv_overrides = vec![];
	let config_toml =
		load_config_as_toml_with_cli_overrides(&codex_home, &cwd, cli_kv_overrides.clone()).await?;

	// 3. Create AuthManager (not wrapped in Arc - it's already Arc internally)
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

	let cloud_requirements =
		cloud_requirements_loader(auth_manager.clone(), chatgpt_base_url, codex_home.clone());

	// 5. Build config using ConfigBuilder
	let mut config = ConfigBuilder::default()
		.codex_home(codex_home.clone())
		.cli_overrides(cli_kv_overrides)
		.cloud_requirements(cloud_requirements.clone())
		.loader_overrides(LoaderOverrides::default())
		.build()
		.await?;

	// 6. Set the model in config (CRITICAL - this is what ChatWidget::new() does)
	let default_model = config.model.clone().or_else(|| Some("mistral-large-latest".to_string()));
	if let Some(model) = &default_model {
		config.model = Some(model.clone());
	}

	// 7. Get auth for session telemetry
	let auth = auth_manager.auth().await;
	let auth_ref = auth.as_ref();

	// 8. Create session telemetry
	let thread_id = codex_protocol::ThreadId::new();
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

	// 10. Create event channels
	let (app_event_tx, app_event_rx) = mpsc::unbounded_channel();
	let app_event_sender = AppEventSender::new(app_event_tx);

	let (frame_tx, _frame_rx) = broadcast::channel(100);
	let frame_requester = FrameRequester::new(frame_tx);

	// 11. Get ModelsManager from ThreadManager
	let models_manager = thread_manager.get_models_manager();

	// 12. Initialize ChatWidget with the model from config
	let default_model = config.model.clone().or_else(|| Some("mistral-large-latest".to_string()));

	let chat_widget_init = ChatWidgetInit {
		config: config.clone(),
		frame_requester,
		app_event_tx: app_event_sender.clone(),
		initial_user_message: None,
		enhanced_keys_supported: true,
		auth_manager: auth_manager.clone(),
		models_manager,
		feedback: codex_feedback::CodexFeedback::new(),
		is_first_run: false,
		feedback_audience: FeedbackAudience::External,
		model: default_model, // Pass the default model
		startup_tooltip_override: None,
		status_line_invalid_items_warned: Arc::new(AtomicBool::new(false)),
		terminal_title_invalid_items_warned: Arc::new(AtomicBool::new(false)),
		session_telemetry,
	};

	// 13. CRITICAL: Start the actual Codex agent thread!
	// This is what makes ChatWidget actually work - without this it just shows "loading" forever
	let codex_op_tx =
		codex_tui_dx::chatwidget::agent::spawn_agent(config.clone(), app_event_sender, thread_manager);

	// 14. Create ChatWidget with the op sender
	let chat_widget =
		codex_tui_dx::chatwidget::ChatWidget::new_with_op_sender(chat_widget_init, codex_op_tx.clone());

	Ok(CodexWidgetState { chat_widget, app_event_rx, codex_op_tx })
}
