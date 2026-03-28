//! Scrollbar widget for the Codex TUI using tui-scrollbar crate
//!
//! This module provides a scrollbar implementation using the tui-scrollbar crate
//! which offers more advanced features and better performance than a custom implementation.

pub use tui_scrollbar::{ScrollBar, ScrollBarArrows, ScrollBarOrientation, ScrollLengths};

use ratatui_core::widgets::Widget;

/// Scrollbar state that tracks scroll position and content dimensions
#[derive(Debug, Clone, Default)]
pub struct ScrollbarState {
	/// Current scroll position (0-based)
	pub position: usize,
	/// Total content height
	pub content_height: usize,
	/// Viewport height
	pub viewport_height: usize,
}

impl ScrollbarState {
	pub fn new(content_height: usize, viewport_height: usize) -> Self {
		Self { position: 0, content_height, viewport_height }
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

	/// Get scroll lengths for tui-scrollbar
	pub fn scroll_lengths(&self) -> ScrollLengths {
		ScrollLengths { content_len: self.content_height, viewport_len: self.viewport_height }
	}
}

/// Custom scrollbar widget wrapper for Codex TUI
#[derive(Debug, Clone)]
pub struct CustomScrollbar {
	/// Whether to show arrows
	show_arrows: bool,
}

impl Default for CustomScrollbar {
	fn default() -> Self {
		Self { show_arrows: false }
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

	/// Render the scrollbar with the given state
	///
	/// This method accepts ratatui types and converts them to ratatui_core types internally
	pub fn render(
		&self,
		area: ratatui::layout::Rect,
		buf: &mut ratatui::buffer::Buffer,
		state: &ScrollbarState,
	) {
		if area.width == 0 || area.height == 0 {
			return;
		}

		let scroll_lengths = state.scroll_lengths();

		let scrollbar = ScrollBar::vertical(scroll_lengths)
			.offset(state.position)
			.glyph_set(tui_scrollbar::GlyphSet::box_drawing()); // Use box drawing for visible track

		let scrollbar =
			if self.show_arrows { scrollbar.arrows(ScrollBarArrows::Both) } else { scrollbar };

		// Convert ratatui types to ratatui_core types
		let core_area =
			ratatui_core::layout::Rect { x: area.x, y: area.y, width: area.width, height: area.height };

		// Create a temporary ratatui_core buffer and render to it
		let mut core_buf = ratatui_core::buffer::Buffer::empty(core_area);
		scrollbar.render(core_area, &mut core_buf);

		// Copy the rendered content back to the ratatui buffer
		for y in 0..core_area.height {
			for x in 0..core_area.width {
				if let Some(core_cell) = core_buf.cell((x, y)) {
					if let Some(cell) = buf.cell_mut(ratatui::layout::Position::new(area.x + x, area.y + y)) {
						cell.set_symbol(core_cell.symbol());
						// Convert style from ratatui_core to ratatui
						let core_style = core_cell.style();
						let mut style = ratatui::style::Style::default();
						if let Some(fg) = core_style.fg {
							style = style.fg(convert_color(fg));
						}
						if let Some(bg) = core_style.bg {
							style = style.bg(convert_color(bg));
						}
						style = style.add_modifier(convert_modifier(core_style.add_modifier));
						style = style.remove_modifier(convert_modifier(core_style.sub_modifier));
						cell.set_style(style);
					}
				}
			}
		}
	}
}

// Helper function to convert ratatui_core::Color to ratatui::Color
fn convert_color(color: ratatui_core::style::Color) -> ratatui::style::Color {
	use ratatui::style::Color as RatatuiColor;
	use ratatui_core::style::Color as CoreColor;

	match color {
		CoreColor::Reset => RatatuiColor::Reset,
		CoreColor::Black => RatatuiColor::Black,
		CoreColor::Red => RatatuiColor::Red,
		CoreColor::Green => RatatuiColor::Green,
		CoreColor::Yellow => RatatuiColor::Yellow,
		CoreColor::Blue => RatatuiColor::Blue,
		CoreColor::Magenta => RatatuiColor::Magenta,
		CoreColor::Cyan => RatatuiColor::Cyan,
		CoreColor::Gray => RatatuiColor::Gray,
		CoreColor::DarkGray => RatatuiColor::DarkGray,
		CoreColor::LightRed => RatatuiColor::LightRed,
		CoreColor::LightGreen => RatatuiColor::LightGreen,
		CoreColor::LightYellow => RatatuiColor::LightYellow,
		CoreColor::LightBlue => RatatuiColor::LightBlue,
		CoreColor::LightMagenta => RatatuiColor::LightMagenta,
		CoreColor::LightCyan => RatatuiColor::LightCyan,
		CoreColor::White => RatatuiColor::White,
		CoreColor::Rgb(r, g, b) => RatatuiColor::Rgb(r, g, b),
		CoreColor::Indexed(i) => RatatuiColor::Indexed(i),
	}
}

// Helper function to convert ratatui_core::Modifier to ratatui::Modifier
fn convert_modifier(modifier: ratatui_core::style::Modifier) -> ratatui::style::Modifier {
	let mut result = ratatui::style::Modifier::empty();

	if modifier.contains(ratatui_core::style::Modifier::BOLD) {
		result |= ratatui::style::Modifier::BOLD;
	}
	if modifier.contains(ratatui_core::style::Modifier::DIM) {
		result |= ratatui::style::Modifier::DIM;
	}
	if modifier.contains(ratatui_core::style::Modifier::ITALIC) {
		result |= ratatui::style::Modifier::ITALIC;
	}
	if modifier.contains(ratatui_core::style::Modifier::UNDERLINED) {
		result |= ratatui::style::Modifier::UNDERLINED;
	}
	if modifier.contains(ratatui_core::style::Modifier::SLOW_BLINK) {
		result |= ratatui::style::Modifier::SLOW_BLINK;
	}
	if modifier.contains(ratatui_core::style::Modifier::RAPID_BLINK) {
		result |= ratatui::style::Modifier::RAPID_BLINK;
	}
	if modifier.contains(ratatui_core::style::Modifier::REVERSED) {
		result |= ratatui::style::Modifier::REVERSED;
	}
	if modifier.contains(ratatui_core::style::Modifier::HIDDEN) {
		result |= ratatui::style::Modifier::HIDDEN;
	}
	if modifier.contains(ratatui_core::style::Modifier::CROSSED_OUT) {
		result |= ratatui::style::Modifier::CROSSED_OUT;
	}

	result
}

// For compatibility with StatefulWidget pattern
impl CustomScrollbar {
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
