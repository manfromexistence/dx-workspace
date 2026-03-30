//! Scrollbar widget for the Codex TUI
//!
//! This module provides a simple, visible scrollbar implementation that renders
//! a track and thumb directly to the buffer.

use ratatui::style::{Color, Style};

/// Scrollbar state that tracks scroll position and content dimensions
#[derive(Debug, Clone, Default)]
pub struct ScrollbarState {
	/// Current scroll position (0-based)
	pub position: usize,
	/// Total content height
	pub content_height: usize,
	/// Viewport height
	pub viewport_height: usize,
	/// Number of transcript markers to show beside the track
	pub marker_count: usize,
}

impl ScrollbarState {
	pub fn new(content_height: usize, viewport_height: usize) -> Self {
		Self { position: 0, content_height, viewport_height, marker_count: 0 }
	}

	pub fn position(mut self, position: usize) -> Self {
		self.position = position;
		self
	}

	pub fn content_height(mut self, height: usize) -> Self {
		self.content_height = height;
		self
	}

	pub fn viewport_height(mut self, height: usize) -> Self {
		self.viewport_height = height;
		self
	}

	pub fn marker_count(mut self, marker_count: usize) -> Self {
		self.marker_count = marker_count;
		self
	}

	/// Scroll up by the given amount
	pub fn scroll_up(&mut self, amount: usize) {
		self.position = self.position.saturating_sub(amount);
	}

	/// Scroll down by the given amount
	pub fn scroll_down(&mut self, amount: usize) {
		let max_scroll = self.content_height.saturating_sub(self.viewport_height);
		self.position = (self.position + amount).min(max_scroll);
	}

	/// Check if we can scroll up
	pub fn can_scroll_up(&self) -> bool {
		self.position > 0
	}

	/// Check if we can scroll down
	pub fn can_scroll_down(&self) -> bool {
		self.position < self.content_height.saturating_sub(self.viewport_height)
	}

	/// Calculate thumb position and size
	pub fn thumb_position_and_size(&self, track_height: usize) -> (usize, usize) {
		if self.content_height == 0 || self.viewport_height == 0 || track_height == 0 {
			return (0, 0);
		}

		if self.content_height <= self.viewport_height {
			return (0, track_height);
		}

		let thumb_size = ((self.viewport_height as f64 / self.content_height as f64) * track_height as f64)
			.max(1.0)
			.min(track_height as f64) as usize;

		let max_scroll = self.content_height.saturating_sub(self.viewport_height);
		let max_thumb_pos = track_height.saturating_sub(thumb_size);
		let thumb_pos = if max_scroll > 0 {
			((self.position as f64 / max_scroll as f64) * max_thumb_pos as f64) as usize
		} else {
			0
		};

		(thumb_pos, thumb_size)
	}
}

/// Custom scrollbar widget for Codex TUI
#[derive(Debug, Clone)]
pub struct CustomScrollbar {
	/// Whether to show arrows
	show_arrows: bool,
	/// Track style
	track_style: Style,
	/// Thumb style
	thumb_style: Style,
}

impl Default for CustomScrollbar {
	fn default() -> Self {
		Self {
			show_arrows: false,
			track_style: Style::default().fg(Color::DarkGray),
			thumb_style: Style::default().fg(Color::White),
		}
	}
}

impl CustomScrollbar {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn show_arrows(mut self, show: bool) -> Self {
		self.show_arrows = show;
		self
	}

	pub fn track_style(mut self, style: Style) -> Self {
		self.track_style = style;
		self
	}

	pub fn thumb_style(mut self, style: Style) -> Self {
		self.thumb_style = style;
		self
	}

	/// Render the scrollbar with the given state
	pub fn render(
		&self,
		area: ratatui::layout::Rect,
		buf: &mut ratatui::buffer::Buffer,
		state: &ScrollbarState,
	) {
		if area.width == 0 || area.height == 0 {
			return;
		}

		let track_width = if area.width > 1 { 1 } else { area.width };
		let track_height = if self.show_arrows {
			area.height.saturating_sub(2) as usize
		} else {
			area.height as usize
		};
		let start_y = if self.show_arrows { 1 } else { 0 };
		let (thumb_pos, thumb_size) = state.thumb_position_and_size(track_height);
		let marker_rows = if state.marker_count > 0 && track_height > 0 {
			(0..state.marker_count)
				.map(|index| ((index + 1) * (track_height + 1)) / (state.marker_count + 1) - 1)
				.collect::<Vec<_>>()
		} else {
			Vec::new()
		};

		if self.show_arrows && area.height >= 2 {
			if let Some(cell) = buf.cell_mut(ratatui::layout::Position::new(area.x, area.y)) {
				cell.set_symbol("▲");
				cell.set_style(self.track_style);
			}
			if let Some(cell) = buf.cell_mut(ratatui::layout::Position::new(area.x, area.y + area.height - 1)) {
				cell.set_symbol("▼");
				cell.set_style(self.track_style);
			}
		}

		for i in 0..track_height {
			let y = area.y + start_y + i as u16;
			if track_width > 0
				&& let Some(cell) = buf.cell_mut(ratatui::layout::Position::new(area.x, y))
			{
				if i >= thumb_pos && i < thumb_pos + thumb_size {
					cell.set_symbol("█");
					cell.set_style(self.thumb_style);
				} else {
					cell.set_symbol("│");
					cell.set_style(self.track_style);
				}
			}

			if area.width > 1
				&& let Some(marker_cell) =
					buf.cell_mut(ratatui::layout::Position::new(area.x + 1, y))
			{
				if marker_rows.contains(&i) {
					let thumb_active = i >= thumb_pos && i < thumb_pos + thumb_size;
					marker_cell.set_symbol(if thumb_active { "◆" } else { "◇" });
					marker_cell.set_style(if thumb_active { self.thumb_style } else { self.track_style });
				} else {
					marker_cell.set_symbol(" ");
					marker_cell.set_style(Style::default());
				}
			}
		}
	}

	/// Render as a stateful widget (for compatibility)
	pub fn render_stateful(
		&self,
		area: ratatui::layout::Rect,
		buf: &mut ratatui::buffer::Buffer,
		state: &mut ScrollbarState,
	) {
		self.render(area, buf, state);
	}
}
