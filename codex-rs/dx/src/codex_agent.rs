// Codex agent spawning and event loop
// Extracted from codex-rs/tui/src/chatwidget/agent.rs
// Adapted for DX TUI - connection logic only, no TUI rendering

use std::sync::Arc;

use codex_core::{CodexThread, Config, NewThread, ThreadManager};
use codex_protocol::protocol::{Event, EventMsg, Op};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

const DX_TUI_CLIENT: &str = "dx-tui";

/// Initialize the app server client name for this TUI
async fn initialize_app_server_client_name(thread: &CodexThread) {
	if let Err(err) = thread.set_app_server_client_name(Some(DX_TUI_CLIENT.to_string())).await {
		tracing::error!("failed to set app server client name: {err}");
	}
}

/// Spawn the Codex agent and return channels for communication
///
/// This function:
/// 1. Starts a new thread with the ThreadManager
/// 2. Spawns an async task that listens for events from Codex
/// 3. Spawns an async task that forwards Ops to Codex
/// 4. Returns (op_tx, event_rx) for the UI to communicate with Codex
///
/// The UI sends Ops via op_tx and receives Events via event_rx.
pub fn spawn_codex_agent(
	config: Config,
	thread_manager: Arc<ThreadManager>,
) -> (UnboundedSender<Op>, UnboundedReceiver<Event>, Arc<CodexThread>) {
	let (codex_op_tx, mut codex_op_rx) = unbounded_channel::<Op>();
	let (event_tx, event_rx) = unbounded_channel::<Event>();

	let event_tx_clone = event_tx.clone();
	let thread_manager_clone = thread_manager.clone();

	tokio::spawn(async move {
		// Start a new Codex thread
		let NewThread { thread, session_configured, .. } =
			match thread_manager_clone.start_thread(config).await {
				Ok(v) => v,
				Err(err) => {
					let message = format!("Failed to initialize codex: {err}");
					tracing::error!("{message}");

					// Send error event to UI
					event_tx_clone.send(Event {
						id: "".to_string(),
						msg: EventMsg::Error(err.to_error_event(/*message_prefix*/ None)),
					}).ok();

					return;
				}
			};

		// Set client name
		initialize_app_server_client_name(thread.as_ref()).await;

		// Forward the SessionConfigured event to UI
		let ev = Event {
			id: "".to_string(),
			msg: EventMsg::SessionConfigured(session_configured),
		};
		event_tx_clone.send(ev).ok();

		// Spawn Op forwarding loop
		let thread_clone = thread.clone();
		tokio::spawn(async move {
			while let Some(op) = codex_op_rx.recv().await {
				let id = thread_clone.submit(op).await;
				if let Err(e) = id {
					tracing::error!("failed to submit op: {e}");
				}
			}
		});

		// Event listening loop
		while let Ok(event) = thread.next_event().await {
			let is_shutdown_complete = matches!(event.msg, EventMsg::ShutdownComplete);

			// Forward event to UI
			event_tx_clone.send(event).ok();

			if is_shutdown_complete {
				// ShutdownComplete is terminal; drop this task
				break;
			}
		}
	});

	(codex_op_tx, event_rx, Arc::new(CodexThread::default()))
}
