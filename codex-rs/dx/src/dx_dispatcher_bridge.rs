//! Bridge between DX dispatcher and Codex ChatWidget
//!
//! This module provides a way to call DX dispatcher methods from ChatWidget
//! without duplicating code. It wraps the DX dispatcher and exposes only
//! the methods needed for timer updates.

use std::cell::RefCell;
use std::time::Duration;
use crate::state::{ChatState, AnimationType};

/// DX Dispatcher Bridge - handles timer updates for ChatWidget
pub struct DxDispatcherBridge {
    // We don't need the full App here, just access to ChatState
}

impl DxDispatcherBridge {
    pub fn new() -> Self {
        Self {}
    }

    /// Call this periodically (e.g., every frame) to update DX state
    /// This is the REAL DX dispatcher timer logic!
    pub fn dispatch_timer(chat_state: &mut ChatState) {
        // Update chat state (process LLM responses, etc.)
        chat_state.update();

        // Update splash font cycling (every 5 seconds)
        // Cycle fonts when in animation mode on Splash screen OR when showing splash in chat mode
        if chat_state.last_font_change.elapsed() >= Duration::from_secs(5) {
            let should_cycle = if chat_state.animation_mode {
                let animations = AnimationType::all();
                let current_anim = animations[chat_state.current_animation_index];
                current_anim == AnimationType::Splash
            } else {
                // In normal chat mode, cycle if showing splash (no messages)
                chat_state.messages.is_empty()
            };

            if should_cycle {
                chat_state.splash_font_index = (chat_state.splash_font_index + 1) % 113; // 113 valid fonts
                chat_state.last_font_change = std::time::Instant::now();
            }
        }

        // Update Menu timing
        let elapsed = chat_state.last_frame_instant.elapsed();
        chat_state.menu.update(elapsed);
        chat_state.last_frame_instant = std::time::Instant::now();
    }
}
