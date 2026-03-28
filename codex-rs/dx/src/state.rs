use crate::{
	autocomplete::Autocomplete,
	codex_integration::CodexWidgetState,
	components::Message,
	effects::{RainbowEffect, ShimmerEffect, TypingIndicator},
	input::InputState,
	llm::LocalLlm,
	menu::Menu,
	models::{ModelInfo, get_default_model},
	perf::PerfMonitor,
	scrollbar::ScrollbarState,
	theme::ChatTheme,
};
use std::{
	path::PathBuf,
	sync::{
		Arc,
		mpsc::{Receiver, Sender, channel},
	},
	time::{Duration, Instant},
};
use tokio::sync::mpsc as tokio_mpsc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationType {
	Splash,
	Matrix,
	Confetti,
	GameOfLife,
	Starfield,
	Rain,
	NyanCat,
	DVDLogo,
	Fire,
	Plasma,
	// Spinners, // COMMENTED OUT: Temporary screen removed
	Waves,
	Fireworks,
	Yazi,
}

impl AnimationType {
	pub fn all() -> Vec<Self> {
		vec![
			Self::Splash, // Start with splash
			Self::Matrix,
			Self::Confetti,
			Self::GameOfLife,
			Self::Starfield,
			Self::Rain,
			Self::NyanCat,
			Self::DVDLogo,
			Self::Fire,
			Self::Plasma,
			// Self::Spinners, // COMMENTED OUT: Temporary screen removed
			Self::Waves,
			Self::Fireworks,
			Self::Yazi, // Last screen
		]
	}

	/// Get only carousel animations (excludes Splash and Yazi)
	pub fn carousel_animations() -> Vec<Self> {
		vec![
			Self::Matrix,
			Self::Confetti,
			Self::GameOfLife,
			Self::Starfield,
			Self::Rain,
			Self::NyanCat,
			Self::DVDLogo,
			Self::Fire,
			Self::Plasma,
			Self::Waves,
			Self::Fireworks,
		]
	}

	#[allow(dead_code)]
	pub fn name(&self) -> &'static str {
		match self {
			Self::Splash => "Splash Screen",
			Self::Matrix => "Matrix Rain",
			Self::Confetti => "Confetti",
			Self::GameOfLife => "Game of Life",
			Self::Starfield => "Starfield",
			Self::Rain => "Rain",
			Self::NyanCat => "Nyan Cat",
			Self::DVDLogo => "DVD Logo",
			Self::Fire => "Fire Animation",
			Self::Plasma => "Plasma Effect",
			// Self::Spinners => "Spinners", // COMMENTED OUT
			Self::Waves => "Ocean Waves",
			Self::Fireworks => "Fireworks",
			Self::Yazi => "Yazi File Manager",
		}
	}
}

pub struct ChatState {
	pub theme: ChatTheme,
	pub theme_mode: crate::theme::ThemeVariant, // Track current theme mode
	pub current_theme_name: String,             // Track current theme name for reloading
	pub input: InputState,
	pub messages: Vec<Message>,
	pub is_loading: bool,
	pub typing_indicator: TypingIndicator,
	pub cursor_visible: bool,
	pub splash_font_index: usize,
	pub last_font_change: Instant,
	pub animation_mode: bool,
	pub current_animation_index: usize,
	pub animation_start_time: Option<Instant>,
	pub llm: Arc<LocalLlm>,
	pub llm_tx: Sender<String>,
	pub llm_rx: Receiver<String>,
	pub llm_status_tx: Sender<String>,
	pub llm_status_rx: Receiver<String>,
	pub show_codex_tui: bool, // Toggle between Ollama chat and Codex TUI
	pub codex_widget: Option<CodexWidgetState>, // Real Codex ChatWidget!
	pub codex_initializing: bool, // Track initialization state
	pub codex_widget_tx: tokio_mpsc::UnboundedSender<CodexWidgetState>, // Channel to send initialized widget
	pub codex_widget_rx: tokio_mpsc::UnboundedReceiver<CodexWidgetState>, // Channel to receive initialized widget
	pub rainbow_animation: RainbowEffect,
	pub rainbow_cursor: RainbowEffect,
	pub shimmer: ShimmerEffect,
	pub last_render: Instant,
	#[allow(dead_code)]
	pub tachyon_last_tick: Duration,
	#[allow(dead_code)]
	pub show_effects_demo_modal: bool,
	pub show_train_animation: bool,
	pub show_matrix_animation: bool,
	pub input_area: ratatui::layout::Rect,
	pub plan_button_area: ratatui::layout::Rect,
	pub model_button_area: ratatui::layout::Rect,
	pub local_button_area: ratatui::layout::Rect,
	pub path_button_area: ratatui::layout::Rect,
	pub paste_button_area: ratatui::layout::Rect, // Track paste button area for click detection
	pub clipboard_buffer: Option<String>,         // Store clipboard content ready to paste
	pub dropped_files: Vec<String>,               // Store paths of dropped files/images
	pub file_button_area: ratatui::layout::Rect,  // Track file button area for click detection
	pub show_dx_splash: bool,
	pub chat_scroll_offset: usize,
	pub chat_area_height: u16, // Track the chat area height for scroll boundary checking
	pub chat_scrollbar_area: ratatui::layout::Rect, // Track scrollbar area for mouse interaction
	pub scrollbar_dragging: bool, // Track if user is dragging the scrollbar
	pub mouse_in_chat_area: bool, // Track if mouse is hovering over chat area
	pub mouse_in_window: bool, // Track if mouse is hovering over the TUI window
	pub input_scroll_offset: usize, // Scroll offset for input box when content exceeds 5 lines
	pub show_attachment_menu: bool, // Track if attachment menu is visible
	pub message_areas: Vec<ratatui::layout::Rect>, // Track message areas for click detection
	pub codex_scroll_offset: usize, // Scroll offset for Codex widget rendering
	pub codex_scrollbar_state: ScrollbarState, // Scrollbar state for Codex TUI
	#[allow(dead_code)]
	pub audio_processing: bool,
	#[allow(dead_code)]
	pub last_shortcut_pressed: Option<String>,
	#[allow(dead_code)]
	pub last_shortcut_time: Instant,
	#[allow(dead_code)]
	pub focus: u8,
	pub shortcut_index: usize,
	pub last_shortcut_cycle: Instant, // Timer for cycling shortcut messages
	#[allow(dead_code)]
	pub mode: u8,
	pub selected_local_mode: String,
	pub selected_model: String,
	pub current_model: ModelInfo, // Current selected model
	pub show_model_picker: bool,  // Show model picker menu
	#[allow(dead_code)]
	pub autocomplete: Autocomplete,
	#[allow(dead_code)]
	pub last_input_change: Instant,
	#[allow(dead_code)]
	pub last_input_content: String,
	pub menu: Menu,
	pub last_frame_instant: Instant,
	#[allow(dead_code)]
	pub show_tachyon_modal: bool,
	pub show_tachyon_menu: bool, // Toggle for menu visibility
	pub menu_is_closing: bool,   // Track if menu is animating closed
	pub perf_monitor: PerfMonitor,
	pub show_perf_overlay: bool,
	#[allow(dead_code)]
	pub last_keystroke_time: Duration,
	pub last_input_render_time: Duration,

	// NEW: File picker integration
	#[allow(dead_code)]
	pub show_file_picker: bool,
	#[allow(dead_code)]
	pub selected_file: Option<PathBuf>,

	// NEW: Intro/Outro animation selection
	pub intro_animation: AnimationType,
	pub outro_animation: AnimationType,

	// NEW: Toast notification system
	pub toast_message: Option<String>,
	pub toast_start_time: Option<Instant>,
	pub toast_duration: Duration,

	// NEW: Transition animation state
	pub playing_intro: bool,
	pub playing_outro: bool,
	pub transition_start_time: Option<Instant>,
	pub transition_duration: Duration,

	// NEW: Space key hold state for spinner
	pub space_held: bool,
	pub space_hold_start: Option<Instant>,
	pub spinner_frame: usize,
	pub last_space_press: Option<Instant>,
	pub space_press_count: usize,

	// NEW: Cursor revert animation
	pub cursor_revert_animation: bool,
	pub cursor_revert_start: Option<Instant>,
	pub cursor_revert_from_pos: usize,

	// NEW: Input focus tracking
	pub input_focused: bool, // Track if input box has focus (starts None, set on first FocusGained)
	pub received_first_focus: bool, // Track if we've received the first focus event

	// NEW: Session file tracking
	pub session_filename: String, // Current session's filename (timestamp-based)
}

impl ChatState {
	pub fn new() -> Self {
		let (llm_tx, llm_rx) = channel();
		let (llm_status_tx, llm_status_rx) = channel();
		let (codex_widget_tx, codex_widget_rx) = tokio_mpsc::unbounded_channel();

		// Try to load DX theme from JSON, fallback to hardcoded if it fails
		let theme_mode = crate::theme::ThemeVariant::Dark;
		let theme = ChatTheme::by_name("dx", theme_mode).unwrap_or_else(ChatTheme::dark_fallback);

		// Get default model and provider name
		let default_model = get_default_model();
		let provider_name = match default_model.provider {
			crate::models::ModelProvider::Local => "Local",
			crate::models::ModelProvider::Codex => "Codex",
		};

		// COMMENTED OUT: Codex widget initialization
		// Start Codex widget initialization in background using spawn_local (works with LocalSet)
		// let codex_tx = codex_widget_tx.clone();
		// tokio::task::spawn_local(async move {
		// 	match crate::codex_integration::initialize_codex_widget().await {
		// 		Ok(widget) => {
		// 			let _ = codex_tx.send(widget);
		// 		}
		// 		Err(_e) => {
		// 			// Initialization failed, widget will remain None
		// 		}
		// 	}
		// });

		let mut state = Self {
			theme: theme.clone(),
			theme_mode,
			current_theme_name: "dx".to_string(),
			input: InputState::new(),
			messages: Vec::new(),
			is_loading: false,
			typing_indicator: TypingIndicator::new(),
			cursor_visible: true,
			splash_font_index: 0,
			last_font_change: Instant::now(),
			animation_mode: false,
			current_animation_index: 0,
			animation_start_time: Some(Instant::now()),
			llm: Arc::new(LocalLlm::new()),
			llm_tx,
			llm_rx,
			llm_status_tx,
			llm_status_rx,
			show_codex_tui: false, // COMMENTED OUT: Start with DX mode, not Codex
			codex_widget: None,
			codex_initializing: false, // COMMENTED OUT: Not initializing
			codex_widget_tx,
			codex_widget_rx,
			rainbow_animation: RainbowEffect::new(),
			rainbow_cursor: RainbowEffect::new(),
			shimmer: ShimmerEffect::new(vec![ratatui::style::Color::Rgb(150, 150, 150)]),
			last_render: Instant::now(),
			tachyon_last_tick: Duration::from_secs(0),
			show_effects_demo_modal: false,
			show_train_animation: false,
			show_matrix_animation: false,
			input_area: ratatui::layout::Rect::default(),
			plan_button_area: ratatui::layout::Rect::default(),
			model_button_area: ratatui::layout::Rect::default(),
			local_button_area: ratatui::layout::Rect::default(),
			path_button_area: ratatui::layout::Rect::default(),
			paste_button_area: ratatui::layout::Rect::default(),
			clipboard_buffer: None,
			dropped_files: Vec::new(),
			file_button_area: ratatui::layout::Rect::default(),
			show_dx_splash: false,
			chat_scroll_offset: 0,
			chat_area_height: 20, // Default height, will be updated during render
			chat_scrollbar_area: ratatui::layout::Rect::default(),
			scrollbar_dragging: false,
			mouse_in_chat_area: false,
			mouse_in_window: false,
			input_scroll_offset: 0,
			show_attachment_menu: false,
			message_areas: Vec::new(),
			codex_scroll_offset: 0,
			codex_scrollbar_state: ScrollbarState::new(0, 0),
			audio_processing: false,
			last_shortcut_pressed: None,
			last_shortcut_time: Instant::now(),
			focus: 0,
			shortcut_index: 0,
			last_shortcut_cycle: Instant::now(),
			mode: 0,
			selected_local_mode: provider_name.to_string(),
			selected_model: default_model.display_name.clone(),
			current_model: default_model, // Start with default (Local Infinity)
			show_model_picker: false,
			autocomplete: Autocomplete::new(theme.clone()),
			last_input_change: Instant::now(),
			last_input_content: String::new(),
			menu: Menu::new(theme),
			last_frame_instant: Instant::now(),
			show_tachyon_modal: false,
			show_tachyon_menu: false, // Start with menu hidden
			menu_is_closing: false,   // Not closing initially
			perf_monitor: PerfMonitor::new(),
			show_perf_overlay: false,
			last_keystroke_time: Duration::from_secs(0),
			last_input_render_time: Duration::from_secs(0),
			show_file_picker: false,
			selected_file: None,
			intro_animation: AnimationType::Matrix, // Default intro animation
			outro_animation: AnimationType::Matrix, // Default outro animation
			toast_message: None,
			toast_start_time: None,
			toast_duration: Duration::from_secs(3), // Toast shows for 3 seconds
			playing_intro: false,
			playing_outro: false,
			transition_start_time: None,
			transition_duration: Duration::from_secs(2), // Transition animations play for 2 seconds
			space_held: false,
			space_hold_start: None,
			spinner_frame: 0,
			last_space_press: None,
			space_press_count: 0,
			cursor_revert_animation: false,
			cursor_revert_start: None,
			cursor_revert_from_pos: 0,
			input_focused: true,        // Start focused by default
			received_first_focus: true, // Assume we have focus at startup
			session_filename: Self::generate_session_filename(), // Generate timestamp-based filename
		};

		// DON'T load messages on startup - always start fresh with splash screen
		// let _ = state.load_messages();

		state
	}

	#[allow(dead_code)]
	pub async fn initialize_llm(&self) {
		if let Err(e) = self.llm.initialize().await {
			eprintln!("Failed to initialize LLM: {}", e);
		}
	}

	#[allow(dead_code)]
	pub fn insert_file_path(&mut self, path: PathBuf) {
		let path_str = path.to_string_lossy();
		self.input.content.push_str(&path_str);
		self.input.cursor_position = self.input.content.len();
		self.selected_file = Some(path);
	}

	#[allow(dead_code)]
	pub fn toggle_file_picker(&mut self) {
		self.show_file_picker = !self.show_file_picker;
	}

	/// Toggle between light and dark theme mode
	pub fn toggle_theme_mode(&mut self) {
		use crate::theme::{ChatTheme, ThemeVariant};

		// Toggle the mode
		self.theme_mode = match self.theme_mode {
			ThemeVariant::Dark => ThemeVariant::Light,
			ThemeVariant::Light => ThemeVariant::Dark,
		};

		// Reload the current theme with the new mode
		if let Some(new_theme) = ChatTheme::by_name(&self.current_theme_name, self.theme_mode) {
			self.theme = new_theme.clone();
			self.menu.theme = new_theme;
		}
	}

	/// Apply a theme by name and mode
	pub fn apply_theme(&mut self, theme_name: &str, mode: crate::theme::ThemeVariant) {
		use crate::theme::ChatTheme;

		if let Some(new_theme) = ChatTheme::by_name(theme_name, mode) {
			self.theme = new_theme.clone();
			self.menu.theme = new_theme;
			self.current_theme_name = theme_name.to_string();
			self.theme_mode = mode;
		}
	}

	pub fn add_user_message(&mut self, content: String) {
		let message = Message::user(content.clone());
		self.messages.push(message);

		// Play intro animation when first message is sent from animation mode
		if self.animation_mode {
			self.animation_mode = false;
			self.play_intro_animation();
		}

		// Reset scroll to bottom
		self.chat_scroll_offset = 0;

		// Start loading and add empty assistant message
		self.is_loading = true;
		self.messages.push(Message::assistant(String::new()));

		// Save messages to disk
		let _ = self.save_messages();

		// Call LLM in background
		let llm = self.llm.clone();
		let tx = self.llm_tx.clone();

		tokio::spawn(async move {
			let tx_clone = tx.clone();
			match llm
				.generate_stream(&content, move |chunk| {
					let _ = tx_clone.send(chunk);
				})
				.await
			{
				Ok(_) => {
					let _ = tx.send("\n__END__".to_string());
				}
				Err(e) => {
					let _ = tx.send(format!("Error: {}", e));
					let _ = tx.send("\n__END__".to_string());
				}
			}
		});
	}

	pub fn update(&mut self) {
		// COMMENTED OUT: Codex TUI initialization check
		// Check for initialized Codex widget
		// if let Ok(widget) = self.codex_widget_rx.try_recv() {
		// 	self.codex_widget = Some(widget);
		// 	self.codex_initializing = false;
		// 	self.show_toast("Codex TUI ready!".to_string());
		// }

		// // Process Codex widget events
		// if let Some(codex_widget) = &mut self.codex_widget {
		// 	codex_widget.process_events();
		// }

		// Process LLM status messages (for toasts)
		if let Ok(status_msg) = self.llm_status_rx.try_recv() {
			self.show_toast(status_msg);
		}

		// Cycle shortcut messages every 10 seconds
		if self.last_shortcut_cycle.elapsed().as_secs() >= 10 {
			self.shortcut_index = (self.shortcut_index + 1) % 3;
			self.last_shortcut_cycle = Instant::now();
		}

		// Hide toast after duration
		if let Some(start_time) = self.toast_start_time {
			if start_time.elapsed() >= self.toast_duration {
				self.toast_message = None;
				self.toast_start_time = None;
			}
		}

		// Handle space key hold spinner with proper hold detection
		if self.space_held {
			if let Some(last_press) = self.last_space_press {
				// If no space press for 150ms, consider it released
				if last_press.elapsed() >= Duration::from_millis(150) {
					self.space_held = false;
					self.space_hold_start = None;
					self.last_space_press = None;
					self.space_press_count = 0;
				} else {
					// Still holding, animate spinner
					if let Some(start_time) = self.space_hold_start {
						let elapsed_ms = start_time.elapsed().as_millis();
						self.spinner_frame = ((elapsed_ms / 100) % 12) as usize;
					}
				}
			}
		}

		// Handle transition animations
		if self.playing_intro || self.playing_outro {
			if let Some(start_time) = self.transition_start_time {
				if start_time.elapsed() >= self.transition_duration {
					// Transition animation finished
					if self.playing_intro {
						self.playing_intro = false;
						self.transition_start_time = None;
						// Animation mode is already off, messages are already added
					} else if self.playing_outro {
						self.playing_outro = false;
						self.transition_start_time = None;
						// Return to splash screen
						self.animation_mode = true;
						self.current_animation_index = 0; // Splash
						self.messages.clear(); // Clear messages
					}
				}
			}
		}

		// Process LLM response chunks
		if let Ok(chunk) = self.llm_rx.try_recv() {
			if chunk == "\n__END__" {
				self.is_loading = false;
				// When streaming ends, collapse thinking accordion if </think> tag is present
				if let Some(last_msg) = self.messages.last_mut()
					&& last_msg.content.contains("</think>")
				{
					last_msg.thinking_expanded = false;
				}
				// Save messages when streaming completes
				let _ = self.save_messages();
			} else if let Some(last_msg) = self.messages.last_mut() {
				last_msg.content.push_str(&chunk);

				// Keep thinking expanded while streaming, but collapse once </think> is received
				if last_msg.content.contains("</think>") {
					last_msg.thinking_expanded = false;
				} else if last_msg.content.contains("<think>") {
					last_msg.thinking_expanded = true;
				}
			}
		}

		// Update typing indicator when loading
		if self.is_loading {
			self.typing_indicator.update();
		}
	}

	/// Show a toast notification
	pub fn show_toast(&mut self, message: String) {
		self.toast_message = Some(message);
		self.toast_start_time = Some(Instant::now());
	}

	/// Start playing intro animation
	pub fn play_intro_animation(&mut self) {
		self.playing_intro = true;
		self.transition_start_time = Some(Instant::now());
		self.animation_start_time = Some(Instant::now());
	}

	/// Start playing outro animation
	pub fn play_outro_animation(&mut self) {
		self.playing_outro = true;
		self.transition_start_time = Some(Instant::now());
		self.animation_start_time = Some(Instant::now());
	}

	/// Calculate maximum scroll offset based on message content
	pub fn calculate_max_scroll(&self) -> usize {
		if self.messages.is_empty() {
			return 0;
		}

		// Calculate total height of all messages (must match MessageList::calculate_total_height)
		let total_height: usize = self
			.messages
			.iter()
			.map(|msg| {
				let content_lines = if msg.content.is_empty() {
					1 // "Thinking..." line
				} else if msg.role == crate::components::MessageRole::Assistant {
					// For assistant messages, we need to parse with thinking accordion
					// to match what MessageList does
					// This is a simplified version - in reality MessageList uses parse_content_with_thinking
					// For now, just count lines as an approximation
					msg.content.lines().count()
				} else {
					// For user messages, just count lines
					msg.content.lines().count()
				};

				match msg.role {
					crate::components::MessageRole::User => {
						// User message: content + header + borders + gap
						content_lines + 3 + 1
					}
					crate::components::MessageRole::Assistant => {
						// Assistant message: content + header + gap (no borders)
						content_lines + 1 + 1
					}
				}
			})
			.sum();

		// Use the actual chat area height from the last render
		// Ensure viewport_height is at least 1 to avoid division issues
		let viewport_height = (self.chat_area_height as usize).max(1);

		// Max scroll is total height minus viewport
		// Don't allow scrolling beyond the content
		total_height.saturating_sub(viewport_height)
	}

	/// Clamp scroll offset to valid range
	pub fn clamp_scroll_offset(&mut self) {
		let max_scroll = self.calculate_max_scroll();
		self.chat_scroll_offset = self.chat_scroll_offset.min(max_scroll);
	}

	/// Generate a timestamp-based filename for the current session
	fn generate_session_filename() -> String {
		let now = chrono::Local::now();
		format!("{}.json", now.format("%Y-%m-%d_%H-%M-%S"))
	}

	/// Get the path to the .dx/tui/history folder (create if doesn't exist)
	fn dx_folder_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
		let mut path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
		path.push(".dx");
		path.push("tui");
		path.push("history");

		// Create .dx/tui/history folder if it doesn't exist
		if !path.exists() {
			std::fs::create_dir_all(&path)?;
		}

		Ok(path)
	}

	/// Get the full path to the current session's messages file
	fn session_file_path(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
		let mut path = Self::dx_folder_path()?;
		path.push(&self.session_filename);
		Ok(path)
	}

	/// Save messages to the current session's JSON file
	pub fn save_messages(&self) -> Result<(), Box<dyn std::error::Error>> {
		let path = self.session_file_path()?;
		let json = serde_json::to_string_pretty(&self.messages)?;
		std::fs::write(path, json)?;
		Ok(())
	}

	/// Load messages from a specific file (not used on startup - kept for potential future use)
	#[allow(dead_code)]
	pub fn load_messages_from_file(
		&mut self,
		filename: &str,
	) -> Result<(), Box<dyn std::error::Error>> {
		let mut path = Self::dx_folder_path()?;
		path.push(filename);
		if path.exists() {
			let json = std::fs::read_to_string(path)?;
			self.messages = serde_json::from_str(&json)?;
		}
		Ok(())
	}

	// COMMENTED OUT: Codex TUI toggle
	// /// Toggle between Ollama chat and Codex TUI
	// pub fn toggle_codex_tui(&mut self) {
	// 	self.show_codex_tui = !self.show_codex_tui;
	// 	let mode = if self.show_codex_tui { "Codex TUI" } else { "Ollama Chat" };
	// 	self.show_toast(format!("Switched to {}", mode));
	// }

	// COMMENTED OUT: Codex TUI initialization
	// /// Initialize Codex TUI App (async)
	// pub async fn initialize_codex_app(&mut self) {
	// 	if self.codex_initializing || self.codex_widget.is_some() {
	// 		return;
	// 	}

	// 	self.codex_initializing = true;
	// 	self.show_toast("Initializing Codex TUI...".to_string());

	// 	match crate::codex_integration::initialize_codex_widget().await {
	// 		Ok(codex_widget) => {
	// 			self.codex_widget = Some(codex_widget);
	// 			self.codex_initializing = false;
	// 			self.show_toast("Codex TUI ready!".to_string());
	// 		}
	// 		Err(e) => {
	// 			self.codex_initializing = false;
	// 			self.show_toast(format!("Failed to initialize Codex: {}", e));
	// 		}
	// 	}
	// }

	/// Toggle model picker menu
	pub fn toggle_model_picker(&mut self) {
		self.show_model_picker = !self.show_model_picker;
	}

	/// Select a model by ID
	pub fn select_model(&mut self, model_id: &str) {
		use crate::models::ModelProvider;

		if let Some(model) = crate::models::get_model_by_id(model_id) {
			self.current_model = model.clone();
			self.show_model_picker = false;

			// Update selected_model to match current model display name
			self.selected_model = model.display_name.clone();

			// Update selected_local_mode based on provider
			self.selected_local_mode = match model.provider {
				ModelProvider::Local => "Local".to_string(),
				ModelProvider::Codex => "Codex".to_string(),
			};

			self.show_toast(format!("Switched to {}", model.display_name));
		}
	}

	/// Get the current model display name
	pub fn current_model_display(&self) -> String {
		self.current_model.display_name.clone()
	}

	/// Get the current provider display name
	pub fn current_provider_display(&self) -> String {
		use crate::models::ModelProvider;
		match self.current_model.provider {
			ModelProvider::Local => "Local".to_string(),
			ModelProvider::Codex => "Codex".to_string(),
		}
	}
}
