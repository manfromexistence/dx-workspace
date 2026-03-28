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
    
    /// Route key events to DX dispatcher logic
    /// This calls the REAL DX key handling code from dispatcher.rs
    pub fn dispatch_key(chat_state: &mut ChatState, key: crossterm::event::KeyEvent) {
        use crossterm::event::{KeyCode, KeyModifiers, KeyEventKind};
        
        // PRIORITY 1: Menu navigation (from dispatcher.rs lines 167-379)
        if chat_state.show_tachyon_menu {
            match key.code {
                KeyCode::Up if key.modifiers.is_empty() || key.modifiers == KeyModifiers::NONE => {
                    chat_state.play_ui_sound("assets/click.mp3");
                    chat_state.menu.select_prev_menu_item();
                    return;
                }
                KeyCode::Char('k') if key.modifiers.is_empty() || key.modifiers == KeyModifiers::NONE => {
                    chat_state.play_ui_sound("assets/click.mp3");
                    chat_state.menu.select_prev_menu_item();
                    return;
                }
                KeyCode::Down if key.modifiers.is_empty() || key.modifiers == KeyModifiers::NONE => {
                    chat_state.play_ui_sound("assets/click.mp3");
                    chat_state.menu.select_next_menu_item();
                    return;
                }
                KeyCode::Char('j') if key.modifiers.is_empty() || key.modifiers == KeyModifiers::NONE => {
                    chat_state.play_ui_sound("assets/click.mp3");
                    chat_state.menu.select_next_menu_item();
                    return;
                }
                KeyCode::PageUp => {
                    chat_state.menu.page_up(10);
                    return;
                }
                KeyCode::PageDown => {
                    chat_state.menu.page_down(10);
                    return;
                }
                KeyCode::Home if key.modifiers.is_empty() || key.modifiers == KeyModifiers::NONE => {
                    chat_state.menu.jump_to_top();
                    return;
                }
                KeyCode::Char('g') if key.modifiers.is_empty() || key.modifiers == KeyModifiers::NONE => {
                    chat_state.menu.jump_to_top();
                    return;
                }
                KeyCode::End if key.modifiers.is_empty() || key.modifiers == KeyModifiers::NONE => {
                    chat_state.menu.jump_to_bottom();
                    return;
                }
                KeyCode::Char('G') if key.modifiers.contains(KeyModifiers::SHIFT) => {
                    chat_state.menu.jump_to_bottom();
                    return;
                }
                KeyCode::Enter => {
                    chat_state.play_ui_sound("assets/click.mp3");
                    let _should_close = !chat_state.menu.select_current_item();
                    chat_state.menu_is_closing = true;
                    chat_state.menu.pick_closing_effect();
                    return;
                }
                KeyCode::Esc => {
                    if chat_state.menu.current_submenu.is_some() {
                        chat_state.menu.go_back_to_main();
                    } else {
                        chat_state.menu_is_closing = true;
                        chat_state.menu.pick_closing_effect();
                    }
                    return;
                }
                _ => {}
            }
        }
        
        // PRIORITY 2: Global '0' key to toggle menu (from dispatcher.rs line 479)
        if key.code == KeyCode::Char('0') && (key.modifiers.is_empty() || key.modifiers == KeyModifiers::NONE) {
            if chat_state.show_tachyon_menu {
                chat_state.menu_is_closing = true;
                chat_state.menu.pick_closing_effect();
                chat_state.play_ui_sound("assets/menu-close.mp3");
            } else {
                chat_state.menu_is_closing = false;
                chat_state.show_tachyon_menu = true;
                chat_state.menu.pick_opening_effect();
                chat_state.play_ui_sound("assets/menu-open.mp3");
            }
            return;
        }
        
        // TODO: Add animation navigation (Left/Right arrows) when implementing animation carousel
        // TODO: Add voice mode (Space key hold) when implementing voice features
    }
}
