use crate::color::blend;
use crate::color::is_light;
use crate::terminal_palette::best_color;
use crate::terminal_palette::default_bg;
use crate::theme::ChatTheme;
use ratatui::style::Color;
use ratatui::style::Style;
use std::sync::{OnceLock, RwLock};

static DX_THEME_OVERRIDE: OnceLock<RwLock<Option<ChatTheme>>> = OnceLock::new();

fn dx_theme_override() -> &'static RwLock<Option<ChatTheme>> {
	DX_THEME_OVERRIDE.get_or_init(|| RwLock::new(None))
}

pub fn set_dx_theme_override(theme: Option<ChatTheme>) {
	if let Ok(mut slot) = dx_theme_override().write() {
		*slot = theme;
	}
}

fn current_dx_theme() -> Option<ChatTheme> {
	dx_theme_override().read().ok().and_then(|theme| theme.clone())
}

pub fn user_message_style() -> Style {
	user_message_style_for(default_bg())
}

pub fn proposed_plan_style() -> Style {
	proposed_plan_style_for(default_bg())
}

/// Returns the style for a user-authored message using the provided terminal background.
pub fn user_message_style_for(terminal_bg: Option<(u8, u8, u8)>) -> Style {
	if let Some(theme) = current_dx_theme() {
		return Style::default().bg(theme.card).fg(theme.card_fg);
	}
	match terminal_bg {
		Some(bg) => Style::default().bg(user_message_bg(bg)),
		None => Style::default(),
	}
}

pub fn proposed_plan_style_for(terminal_bg: Option<(u8, u8, u8)>) -> Style {
	if let Some(theme) = current_dx_theme() {
		return Style::default().bg(theme.card).fg(theme.card_fg);
	}
	match terminal_bg {
		Some(bg) => Style::default().bg(proposed_plan_bg(bg)),
		None => Style::default(),
	}
}

pub fn dx_accent_color() -> Color {
	current_dx_theme().map_or(Color::Cyan, |theme| theme.accent)
}

#[allow(clippy::disallowed_methods)]
pub fn user_message_bg(terminal_bg: (u8, u8, u8)) -> Color {
	let (top, alpha) =
		if is_light(terminal_bg) { ((0, 0, 0), 0.04) } else { ((255, 255, 255), 0.12) };
	best_color(blend(top, terminal_bg, alpha))
}

#[allow(clippy::disallowed_methods)]
pub fn proposed_plan_bg(terminal_bg: (u8, u8, u8)) -> Color {
	user_message_bg(terminal_bg)
}
