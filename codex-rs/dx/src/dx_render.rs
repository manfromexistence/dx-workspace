use codex_tui_dx::render::renderable::Renderable;
use ratatui::{
	buffer::Buffer,
	layout::{Alignment, Constraint, Direction, Layout, Rect},
	widgets::Widget,
};

use super::{
	components::MessageList,
	splash,
	state::{AnimationType, ChatState},
};

impl ChatState {
	pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
		// Update tachyon effects timing
		let _elapsed = self.last_render.elapsed();

		// Calculate dynamic input height
		let input_height = self.calculate_input_height(area.width);

		// PRIORITY 0: Playing intro/outro transition animations
		if self.playing_intro || self.playing_outro {
			// Split screen: animation area + input + controls (like animation carousel)
			let chunks = Layout::default()
				.direction(Direction::Vertical)
				.constraints([Constraint::Min(10), Constraint::Length(input_height), Constraint::Length(1)])
				.split(area);

			self.input_area = chunks[1];

			// Clear the animation area first
			for y in chunks[0].top()..chunks[0].bottom() {
				for x in chunks[0].left()..chunks[0].right() {
					buf[(x, y)].reset();
					buf[(x, y)].set_bg(self.theme_bg_color());
				}
			}

			// Render the appropriate transition animation
			let anim_type = if self.playing_intro { self.intro_animation } else { self.outro_animation };

			match anim_type {
				AnimationType::Matrix => {
					self.render_matrix_animation_in_area(chunks[0], buf);
				}
				AnimationType::Confetti => {
					self.render_confetti_animation_in_area(chunks[0], buf);
				}
				AnimationType::GameOfLife => {
					self.render_gameoflife_animation_in_area(chunks[0], buf);
				}
				AnimationType::Starfield => {
					self.render_starfield_animation_in_area(chunks[0], buf);
				}
				AnimationType::Rain => {
					self.render_rain_animation_in_area(chunks[0], buf);
				}
				AnimationType::NyanCat => {
					self.render_nyancat_animation_in_area(chunks[0], buf);
				}
				AnimationType::DVDLogo => {
					self.render_dvdlogo_animation_in_area(chunks[0], buf);
				}
				AnimationType::Fire => {
					self.render_fire_animation_in_area(chunks[0], buf);
				}
				AnimationType::Plasma => {
					self.render_plasma_animation_in_area(chunks[0], buf);
				}
				AnimationType::Waves => {
					self.render_waves_animation_in_area(chunks[0], buf);
				}
				AnimationType::Fireworks => {
					self.render_fireworks_animation_in_area(chunks[0], buf);
				}
				_ => {
					// For Splash or Yazi, just show a blank screen during transition
				}
			}

			// Render input box and bottom controls
			self.render_input_box(chunks[1], buf);
			let (plan_area, model_area, _token_area, local_area) =
				self.render_bottom_controls(chunks[2], buf);
			self.plan_button_area = plan_area;
			self.model_button_area = model_area;
			self.local_button_area = local_area;

			// Render toast notification (on top of everything)
			self.render_toast(area, buf);
			return;
		}

		// Both animations show in full screen, no input or controls
		if self.show_train_animation || self.show_matrix_animation {
			// Clear the entire area first
			for y in area.top()..area.bottom() {
				for x in area.left()..area.right() {
					buf[(x, y)].reset();
					buf[(x, y)].set_bg(self.theme_bg_color());
				}
			}

			// Render appropriate animation in the full area
			if self.show_train_animation {
				self.render_train_animation_in_area(area, buf);
			} else if self.show_matrix_animation {
				self.render_matrix_animation_in_area(area, buf);
			}
			return;
		}

		// Animation carousel mode
		if self.animation_mode {
			let animations = AnimationType::all();
			let current_anim = animations[self.current_animation_index];

			// Special handling for Yazi screen - don't render here, let root handle it
			if current_anim == AnimationType::Yazi {
				// Just render input and controls, yazi will be rendered by root widget
				let chunks = Layout::default()
					.direction(Direction::Vertical)
					.constraints([
						Constraint::Min(10),
						Constraint::Length(input_height),
						Constraint::Length(1),
					])
					.split(area);

				self.input_area = chunks[1];
				self.render_input_box(chunks[1], buf);
				let (plan_area, model_area, _token_area, local_area) =
					self.render_bottom_controls(chunks[2], buf);
				self.plan_button_area = plan_area;
				self.model_button_area = model_area;
				self.local_button_area = local_area;
				return;
			}

			// Matrix animation - show with input and controls
			if current_anim == AnimationType::Matrix {
				let chunks = Layout::default()
					.direction(Direction::Vertical)
					.constraints([
						Constraint::Min(10),
						Constraint::Length(input_height),
						Constraint::Length(1),
					])
					.split(area);

				self.input_area = chunks[1];

				// Clear the animation area first
				for y in chunks[0].top()..chunks[0].bottom() {
					for x in chunks[0].left()..chunks[0].right() {
						buf[(x, y)].reset();
						buf[(x, y)].set_bg(self.theme_bg_color());
					}
				}

				// Render animation in the main area
				self.render_matrix_animation_in_area(chunks[0], buf);

				// Render intro/outro indicators (top-left corner)
				self.render_animation_indicators(chunks[0], current_anim, buf);

				// Render input box and bottom controls
				self.render_input_box(chunks[1], buf);
				let (plan_area, model_area, _token_area, local_area) =
					self.render_bottom_controls(chunks[2], buf);
				self.plan_button_area = plan_area;
				self.model_button_area = model_area;
				self.local_button_area = local_area;

				// Render menu overlay if visible
				if self.show_tachyon_menu || self.menu_is_closing {
					self.render_menu_in_area(area, buf);
				}

				// Render toast notification (on top of everything)
				self.render_toast(area, buf);
				return;
			}

			let chunks = Layout::default()
				.direction(Direction::Vertical)
				.constraints([Constraint::Min(10), Constraint::Length(input_height), Constraint::Length(1)])
				.split(area);

			self.input_area = chunks[1];

			// Render the current animation in the chat area
			match current_anim {
				AnimationType::Splash => {
					splash::render(
						chunks[0],
						buf,
						&self.theme,
						self.splash_font_index,
						&self.rainbow_animation,
					);
				}
				AnimationType::Matrix => {
					// Already handled above
				}
				AnimationType::Confetti => {
					self.render_confetti_animation_in_area(chunks[0], buf);
				}
				AnimationType::GameOfLife => {
					self.render_gameoflife_animation_in_area(chunks[0], buf);
				}
				AnimationType::Starfield => {
					self.render_starfield_animation_in_area(chunks[0], buf);
				}
				AnimationType::Rain => {
					self.render_rain_animation_in_area(chunks[0], buf);
				}
				AnimationType::NyanCat => {
					self.render_nyancat_animation_in_area(chunks[0], buf);
				}
				AnimationType::DVDLogo => {
					self.render_dvdlogo_animation_in_area(chunks[0], buf);
				}
				AnimationType::Fire => {
					self.render_fire_animation_in_area(chunks[0], buf);
				}
				AnimationType::Plasma => {
					self.render_plasma_animation_in_area(chunks[0], buf);
				}
				// AnimationType::Spinners => {
				// 	self.render_spinners_animation_in_area(chunks[0], buf);
				// } // COMMENTED OUT: Temporary screen removed
				AnimationType::Waves => {
					self.render_waves_animation_in_area(chunks[0], buf);
				}
				AnimationType::Fireworks => {
					self.render_fireworks_animation_in_area(chunks[0], buf);
				}
				AnimationType::Yazi => {
					// Exit animation mode and show yazi file picker
					// This will be handled by the root widget
					return;
				}
			}

			// Render intro/outro indicators (top-left corner)
			self.render_animation_indicators(chunks[0], current_anim, buf);

			// Render input box and bottom controls
			self.render_input_box(chunks[1], buf);

			let (plan_area, model_area, _token_area, local_area) =
				self.render_bottom_controls(chunks[2], buf);

			self.plan_button_area = plan_area;
			self.model_button_area = model_area;
			self.local_button_area = local_area;

			// Render menu overlay if visible (on top of animations)
			if self.show_tachyon_menu || self.menu_is_closing {
				self.render_menu_in_area(area, buf);
			}

			// Render toast notification (on top of everything)
			self.render_toast(area, buf);
			return;
		}

		if self.show_dx_splash {
			// Show DX splash screen
			splash::render(area, buf, &self.theme, self.splash_font_index, &self.rainbow_animation);
			return;
		}

		// COMMENTED OUT: Codex TUI rendering
		// When Codex TUI is active, render it with DX bottom controls
		// if self.show_codex_tui {
		// 	if let Some(codex_widget) = &mut self.codex_widget {
		// 		// Split: Codex TUI area + DX bottom controls (1 line)
		// 		let chunks = Layout::default()
		// 			.direction(Direction::Vertical)
		// 			.constraints([
		// 				Constraint::Min(10),     // Codex TUI (rest of space)
		// 				Constraint::Length(1),   // DX bottom controls (1 line)
		// 			])
		// 			.split(area);

		// 		let codex_area = chunks[0];
		// 		let controls_area = chunks[1];

		// 		// Process events BEFORE rendering
		// 		codex_widget.process_events();
		//
		// 		// CRITICAL: Call pre_draw_tick() before rendering (like real codex-tui-dx does)
		// 		codex_widget.chat_widget.pre_draw_tick();
		//
		// 		// Render ChatWidget in the codex area
		// 		// It will handle its own layout: history area + input composer + status line
		// 		codex_widget.chat_widget.render(codex_area, buf);
		//
		// 		// Render DX bottom controls
		// 		let (plan_area, model_area, _token_area, local_area) =
		// 			self.render_bottom_controls(controls_area, buf);
		// 		self.plan_button_area = plan_area;
		// 		self.model_button_area = model_area;
		// 		self.local_button_area = local_area;
		// 	} else if self.codex_initializing {
		if self.show_codex_tui {
			if self.codex_initializing {
				// Show initializing message in a smaller area, keep DX controls
				let chunks = Layout::default()
					.direction(Direction::Vertical)
					.constraints([
						Constraint::Min(10),
						Constraint::Length(input_height),
						Constraint::Length(1),
					])
					.split(area);

				self.input_area = chunks[1];

				// Show initializing message
				use ratatui::{
					style::{Modifier, Style},
					text::{Line, Span},
					widgets::{Block, Borders, Paragraph},
				};

				let init_widget = Paragraph::new(vec![
					Line::from(""),
					Line::from(Span::styled(
						"Initializing Codex TUI...",
						Style::default().fg(self.theme.accent).add_modifier(Modifier::BOLD),
					)),
					Line::from(""),
					Line::from(Span::styled(
						"Loading configuration and starting agent...",
						Style::default().fg(self.theme.fg),
					)),
				])
				.style(Style::default().bg(self.theme.bg))
				.alignment(Alignment::Center)
				.block(
					Block::default()
						.borders(Borders::ALL)
						.border_style(Style::default().fg(self.theme.border)),
				);
				init_widget.render(chunks[0], buf);

				// Render DX input box and controls while initializing
				self.render_input_box(chunks[1], buf);
				let (plan_area, model_area, _token_area, local_area) =
					self.render_bottom_controls(chunks[2], buf);
				self.plan_button_area = plan_area;
				self.model_button_area = model_area;
				self.local_button_area = local_area;
			}
		} else {
			// DX mode: Split screen with DX input/controls at bottom
			let chunks = Layout::default()
				.direction(Direction::Vertical)
				.constraints([Constraint::Min(10), Constraint::Length(input_height), Constraint::Length(1)])
				.split(area);

			self.input_area = chunks[1];

			// Update chat area height for scroll boundary checking
			self.chat_area_height = chunks[0].height;

			// Clamp scroll offset to valid range after updating chat_area_height
			self.clamp_scroll_offset();

			// Track scrollbar area (right edge of chat area with wider hit area for easier interaction)
			self.chat_scrollbar_area = ratatui::layout::Rect {
				x: chunks[0].x + chunks[0].width.saturating_sub(2), // 2 columns wide for easier clicking
				y: chunks[0].y,
				width: 2,
				height: chunks[0].height,
			};

			if self.messages.is_empty() {
				// Show splash when no messages
				splash::render(
					chunks[0],
					buf,
					&self.theme,
					self.splash_font_index,
					&self.rainbow_animation,
				);
			} else {
				// Show DX message list
				MessageList::with_effects(
					&self.messages,
					&self.theme,
					self.chat_scroll_offset,
					&self.shimmer,
					&self.typing_indicator,
					&mut self.message_areas,
					self.mouse_in_window,
				)
				.render(chunks[0], buf);
			}

			self.render_input_box(chunks[1], buf);

			let (plan_area, model_area, _token_area, local_area) =
				self.render_bottom_controls(chunks[2], buf);

			self.plan_button_area = plan_area;
			self.model_button_area = model_area;
			self.local_button_area = local_area;
		}

		// Render performance overlay if enabled
		self.render_perf_overlay(area, buf);

		// Render menu overlay globally if visible (on top of everything)
		if self.show_tachyon_menu || self.menu_is_closing {
			self.render_menu_in_area(area, buf);
		}

		// Render toast notification (on top of everything)
		self.render_toast(area, buf);
	}

	pub fn render_dimmed(&mut self, area: Rect, full_area: Rect, buf: &mut Buffer) {
		// Simplified render for FilePicker mode - just show input box and controls
		// Calculate dynamic input height
		let input_height = self.calculate_input_height(area.width);

		// Split into input (dynamic height) and controls (1 line)
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints([
				Constraint::Length(input_height), // Input box (dynamic height)
				Constraint::Length(1),            // Bottom controls (1 line)
			])
			.split(area);

		self.input_area = chunks[0];

		// Render input box
		self.render_input_box(chunks[0], buf);

		// Render bottom controls
		let (plan_area, model_area, _token_area, local_area) =
			self.render_bottom_controls(chunks[1], buf);

		self.plan_button_area = plan_area;
		self.model_button_area = model_area;
		self.local_button_area = local_area;

		// Render menu overlay globally if visible (on top of everything)
		// Use full_area to center menu in the entire terminal, not just the chat area
		if self.show_tachyon_menu || self.menu_is_closing {
			self.render_menu_in_area(full_area, buf);
		}

		// Render toast notification (on top of everything)
		self.render_toast(full_area, buf);
	}
}

// Input rendering methods
use ratatui::{
	style::{Modifier, Style},
	text::{Line, Span, Text},
	widgets::{
		Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget,
		Wrap,
	},
};

impl ChatState {
	/// Calculate the required height for the input box based on content
	/// Returns a value between 3 (minimum) and 7 (maximum: 5 lines of text + 2 for borders)
	fn calculate_input_height(&self, available_width: u16) -> u16 {
		if self.input.content.is_empty() {
			return 3; // Minimum height: 1 line + 2 for borders
		}

		// Calculate how many lines the content will take when wrapped
		// Subtract 4 for borders (2) and horizontal padding (2)
		let text_width = available_width.saturating_sub(4) as usize;
		if text_width == 0 {
			return 3;
		}

		let mut line_count = 0;
		for line in self.input.content.lines() {
			if line.is_empty() {
				line_count += 1;
			} else {
				// Calculate wrapped lines for this line
				let chars = line.chars().count();
				line_count += (chars + text_width - 1) / text_width; // Ceiling division
			}
		}

		// Add 2 for borders, clamp between 3 and 7
		(line_count as u16 + 2).clamp(3, 7)
	}

	pub fn render_input_box(&mut self, area: Rect, buf: &mut Buffer) {
		// Start timing input render
		self.perf_monitor.start_timing();

		let block = Block::default()
			.borders(Borders::ALL)
			.border_style(Style::default().fg(self.theme.border))
			.border_type(ratatui::widgets::BorderType::Rounded)
			.style(Style::default()); // Transparent background - no bg color set

		let inner = block.inner(area);
		block.render(area, buf);

		// Add horizontal padding inside the input box
		let padded_inner = Rect {
			x: inner.x + 1,
			y: inner.y,
			width: inner.width.saturating_sub(2),
			height: inner.height,
		};

		self.render_input_text(padded_inner, buf);
		self.render_input_cursor(padded_inner, buf);

		// Render scrollbar if content exceeds viewport height
		self.render_input_scrollbar(inner, buf);

		// Render spinner on the right side if space is held
		if self.space_held {
			self.render_input_spinner(padded_inner, buf);
		}

		// Record input render time
		self.last_input_render_time = self.perf_monitor.record_input_render();
	}

	fn render_input_scrollbar(&self, area: Rect, buf: &mut Buffer) {
		// Calculate total lines in input
		let text_width = area.width.saturating_sub(4) as usize; // Subtract borders and padding
		if text_width == 0 {
			return;
		}

		let mut total_lines = 0;
		for line in self.input.content.lines() {
			if line.is_empty() {
				total_lines += 1;
			} else {
				let chars = line.chars().count();
				total_lines += (chars + text_width - 1) / text_width;
			}
		}

		// Show scrollbar if content exceeds viewport height
		let viewport_height = area.height.saturating_sub(2) as usize; // Subtract borders
		if total_lines > viewport_height {
			let max_scroll = total_lines.saturating_sub(viewport_height);

			let mut scrollbar_state = ScrollbarState::new(max_scroll)
				.position(self.input_scroll_offset)
				.viewport_content_length(viewport_height);

			let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
				.style(Style::default().fg(self.theme.border))
				.track_symbol(None)
				.thumb_symbol("█")
				.begin_symbol(None)
				.end_symbol(None);

			StatefulWidget::render(scrollbar, area, buf, &mut scrollbar_state);
		}
	}

	fn render_input_text(&self, area: Rect, buf: &mut Buffer) {
		let placeholder = "A question or a prompt... (Enter to send, Shift+Enter for new line)";

		if self.input.content.is_empty() {
			let text =
				Text::from(Line::from(Span::styled(placeholder, Style::default().fg(self.theme.border))));
			Paragraph::new(text)
				.wrap(Wrap { trim: false })
				.style(Style::default().fg(self.theme.fg))
				.render(area, buf);
			return;
		}

		// Calculate wrapped lines
		let text_width = area.width as usize;
		if text_width == 0 {
			return;
		}

		let mut wrapped_lines = Vec::new();
		for line in self.input.content.lines() {
			if line.is_empty() {
				wrapped_lines.push(String::new());
			} else {
				let chars: Vec<char> = line.chars().collect();
				let mut current_line = String::new();

				for ch in chars {
					current_line.push(ch);
					if current_line.chars().count() >= text_width {
						wrapped_lines.push(current_line.clone());
						current_line.clear();
					}
				}

				if !current_line.is_empty() || line.is_empty() {
					wrapped_lines.push(current_line);
				}
			}
		}

		// Apply scroll offset - skip lines that are scrolled out of view
		let visible_lines: Vec<String> =
			wrapped_lines.into_iter().skip(self.input_scroll_offset).take(area.height as usize).collect();

		// Render visible lines
		if self.input.has_selection() {
			self.render_selection(area, buf);
		} else {
			let text: Vec<Line> = visible_lines.iter().map(|s| Line::from(s.as_str())).collect();
			Paragraph::new(text).style(Style::default().fg(self.theme.fg)).render(area, buf);
		}
	}

	fn render_selection(&self, area: Rect, buf: &mut Buffer) {
		let (sel_start, sel_end) =
			if let (Some(start), Some(end)) = (self.input.selection_start, self.input.selection_end) {
				if start < end { (start, end) } else { (end, start) }
			} else {
				(0, 0)
			};

		// Calculate wrapped lines with selection
		let text_width = area.width as usize;
		if text_width == 0 {
			return;
		}

		let mut wrapped_lines = Vec::new();
		let mut char_positions = Vec::new(); // Track character index for each position
		let mut char_idx = 0;

		for line in self.input.content.lines() {
			if line.is_empty() {
				wrapped_lines.push(String::new());
				char_positions.push(vec![char_idx]);
				char_idx += 1; // Account for newline
			} else {
				let chars: Vec<char> = line.chars().collect();
				let mut current_line = String::new();
				let mut current_positions = Vec::new();

				for ch in chars {
					current_line.push(ch);
					current_positions.push(char_idx);
					char_idx += 1;

					if current_line.chars().count() >= text_width {
						wrapped_lines.push(current_line.clone());
						char_positions.push(current_positions.clone());
						current_line.clear();
						current_positions.clear();
					}
				}

				if !current_line.is_empty() || line.is_empty() {
					wrapped_lines.push(current_line);
					char_positions.push(current_positions);
				}
				char_idx += 1; // Account for newline
			}
		}

		// Apply scroll offset and render visible lines with selection
		let visible_lines: Vec<(String, Vec<usize>)> = wrapped_lines
			.into_iter()
			.zip(char_positions.into_iter())
			.skip(self.input_scroll_offset)
			.take(area.height as usize)
			.collect();

		for (line_idx, (line, positions)) in visible_lines.iter().enumerate() {
			let y = area.y + line_idx as u16;
			if y >= area.bottom() {
				break;
			}

			for (col_idx, (ch, &char_pos)) in line.chars().zip(positions.iter()).enumerate() {
				let x = area.x + col_idx as u16;
				if x >= area.right() {
					break;
				}

				let is_selected = char_pos >= sel_start && char_pos < sel_end;
				let style = if is_selected {
					Style::default().bg(self.theme.fg).fg(self.theme.bg)
				} else {
					Style::default().fg(self.theme.fg)
				};

				let cell = &mut buf[(x, y)];
				cell.set_char(ch);
				cell.set_style(style);
			}
		}
	}

	fn render_input_cursor(&self, area: Rect, buf: &mut Buffer) {
		// Don't render cursor until we've received the first focus event
		if !self.received_first_focus {
			return;
		}

		if self.cursor_visible {
			// Calculate cursor position accounting for wrapping and scroll
			let text_width = area.width as usize;
			if text_width == 0 {
				return;
			}

			let cursor_pos = self.input.cursor_position;
			let content = &self.input.content;

			// Calculate which line and column the cursor is on
			let mut current_pos = 0;
			let mut cursor_line = 0;
			let mut cursor_col = 0;

			for line in content.lines() {
				let line_len = line.len();

				if current_pos + line_len >= cursor_pos {
					// Cursor is on this line
					let pos_in_line = cursor_pos - current_pos;
					cursor_col = pos_in_line % text_width;
					cursor_line += pos_in_line / text_width;
					break;
				} else {
					// Move past this line
					current_pos += line_len + 1; // +1 for newline
					if line.is_empty() {
						cursor_line += 1;
					} else {
						let chars = line.chars().count();
						cursor_line += (chars + text_width - 1) / text_width;
					}
				}
			}

			// Apply scroll offset
			if cursor_line < self.input_scroll_offset {
				return; // Cursor is scrolled out of view (above)
			}
			let visible_line = cursor_line - self.input_scroll_offset;

			// Check if cursor is within visible area
			if visible_line >= area.height as usize {
				return; // Cursor is scrolled out of view (below)
			}

			let cursor_x = area.x + cursor_col as u16;
			let cursor_y = area.y + visible_line as u16;

			if cursor_x < area.right() && cursor_y < area.bottom() {
				let cell = &mut buf[(cursor_x, cursor_y)];
				let existing_char = cell.symbol().chars().next().unwrap_or(' ');

				if self.input_focused {
					// Focused: rainbow animated cursor
					let rainbow_color = self.rainbow_cursor.current_color();

					if existing_char == ' ' || self.input.content.is_empty() {
						cell.set_char('▎');
						cell.set_style(Style::default().fg(rainbow_color));
					} else {
						cell.set_style(Style::default().bg(rainbow_color).fg(self.theme.bg));
					}
				} else {
					// Not focused: dim static cursor (no animation)
					let dim_color = self.theme.border; // Use border color for unfocused state

					if existing_char == ' ' || self.input.content.is_empty() {
						cell.set_char('▎');
						cell.set_style(Style::default().fg(dim_color));
					} else {
						cell.set_style(Style::default().bg(dim_color).fg(self.theme.bg));
					}
				}
			}
		}
	}

	fn render_input_spinner(&self, area: Rect, buf: &mut Buffer) {
		// Block spinner frames
		let spinner_frames = ['▁', '▃', '▄', '▅', '▆', '▇', '█', '▇', '▆', '▅', '▄', '▃'];
		let frame_char = spinner_frames[self.spinner_frame % spinner_frames.len()];

		// Position spinner on the far right inside the input box
		// area is the inner area (already inside the border)
		let spinner_x = area.right().saturating_sub(1); // 1 char from right edge
		let spinner_y = area.y + (area.height / 2); // Vertically centered

		if spinner_x < area.right() && spinner_y < area.bottom() {
			let cell = &mut buf[(spinner_x, spinner_y)];

			// Use rainbow color for the spinner
			let color = self.rainbow_animation.current_color();

			cell.set_char(frame_char);
			cell.set_style(Style::default().fg(color).add_modifier(Modifier::BOLD));
		}
	}

	pub fn render_bottom_controls(
		&mut self,
		area: Rect,
		buf: &mut Buffer,
	) -> (Rect, Rect, Rect, Rect) {
		// Context-aware shortcuts based on current screen
		let shortcuts = if self.animation_mode {
			let animations = AnimationType::all();
			let current_anim = animations[self.current_animation_index];

			if current_anim == AnimationType::Yazi {
				// File Browser tips
				[
					"Left/Right Arrow: Return to Splash | Navigate files with arrows",
					"Enter: Select file | Tab: Switch panes | /: Search",
					"Space: Select multiple | d: Delete | r: Rename | y: Copy",
				]
			} else if current_anim == AnimationType::Splash {
				// Splash screen tips
				[
					"Right Arrow: File Browser | Left Arrow: Animation Carousel",
					"Type a message and press Enter to start chatting",
					"0/Ctrl+P: Command Palette | Ctrl+T: Theme | Ctrl+C: Exit",
				]
			} else {
				// Animation Carousel tips
				[
					"Up Arrow: Set as INTRO animation | Down Arrow: Set as OUTRO animation",
					"Left Arrow: Previous animation | Right Arrow: Return to Splash",
					"Intro plays when entering chat | Outro plays when exiting (Ctrl+C)",
				]
			}
		} else {
			// Normal chat mode tips (rotating)
			[
				"0/Ctrl+P: Toggle Command Palette | Space(Hold): Voice Input",
				"Left/Right Arrow: Explore Screens | Ctrl+C: Exit to Splash",
				"1/2/3/(Numbers): Toggle Menus | Ctrl+T: Theme",
			]
		};

		let current_shortcut = if self.animation_mode {
			// In animation mode, show all tips without rotation
			let animations = AnimationType::all();
			let current_anim = animations[self.current_animation_index];
			if current_anim == AnimationType::Yazi || current_anim == AnimationType::Splash {
				// Show first tip for Yazi and Splash
				shortcuts[0]
			} else {
				// Rotate tips for animation carousel
				shortcuts[self.shortcut_index % shortcuts.len()]
			}
		} else {
			// Normal rotation for chat mode
			shortcuts[self.shortcut_index % shortcuts.len()]
		};

		let mode_text = "Agent"; // Simplified for minimal version

		let local_width = self.selected_local_mode.len() as u16;
		let mode_width = mode_text.len() as u16;
		let model_width = self.selected_model.len() as u16;

		// Calculate token usage
		let total_tokens: usize = self
            .messages
            .iter()
            .map(|msg| msg.content.len() / 4) // Rough estimate: 1 token ≈ 4 chars
            .sum();

		// Use current model's context window, or default to 128K
		let context_limit = if self.current_model.is_unlimited {
			999_999_999 // Effectively unlimited
		} else {
			self.current_model.context_window.unwrap_or(128_000)
		};

		let token_ratio = if context_limit > 0 && context_limit < 999_999_999 {
			(total_tokens as f32 / context_limit as f32 * 100.0) as u32
		} else {
			0
		};

		let token_info = if self.current_model.is_unlimited {
			format!("{:.1}K/∞", total_tokens as f32 / 1000.0)
		} else {
			format!("{:.1}K/{}K({}%)", total_tokens as f32 / 1000.0, context_limit / 1000, token_ratio)
		};
		let token_width = token_info.len() as u16;

		// Get current working directory and truncate
		let cwd = std::env::current_dir()
			.ok()
			.and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
			.unwrap_or_else(|| "~".to_string());
		let path_info = format!("/{}", cwd);
		let path_width = path_info.len().min(20) as u16; // Truncate to max 20 chars
		let truncated_path = if path_info.len() > 20 {
			format!("..{}", &path_info[path_info.len() - 17..])
		} else {
			path_info.clone()
		};

		let spinner_width = if self.is_loading { 2 } else { 0 };

		let mut constraints = vec![
			Constraint::Length(local_width),
			Constraint::Length(1),
			Constraint::Length(mode_width),
			Constraint::Length(1),
			Constraint::Length(model_width),
			Constraint::Min(10),
			Constraint::Length(token_width),
			Constraint::Length(1),
			Constraint::Length(path_width),
		];

		if self.is_loading {
			constraints.push(Constraint::Length(1));
			constraints.push(Constraint::Length(spinner_width));
		}

		let bottom_chunks =
			Layout::default().direction(Direction::Horizontal).constraints(constraints).split(area);

		Paragraph::new(Span::styled(&self.selected_local_mode, Style::default().fg(self.theme.fg)))
			.alignment(ratatui::layout::Alignment::Left)
			.render(bottom_chunks[0], buf);

		Paragraph::new(Span::styled(mode_text, Style::default().fg(self.theme.fg)))
			.alignment(ratatui::layout::Alignment::Left)
			.render(bottom_chunks[2], buf);

		Paragraph::new(Span::styled(&self.selected_model, Style::default().fg(self.theme.fg)))
			.alignment(ratatui::layout::Alignment::Left)
			.render(bottom_chunks[4], buf);

		// Render action center (buttons or shortcuts) in the center area
		let center_area = bottom_chunks[5];

		// Check if we have any buttons to show
		let has_paste = self.clipboard_buffer.is_some();
		let has_files = !self.dropped_files.is_empty();

		if has_paste || has_files {
			// Render file and paste info in the center with center dot separator
			let mut button_spans = Vec::new();

			// Add file info first (if present)
			if has_files {
				// Extract just the filename from the first file path
				let first_file = &self.dropped_files[0];
				let filename = std::path::Path::new(first_file)
					.file_name()
					.and_then(|n| n.to_str())
					.unwrap_or(first_file);

				let file_count = self.dropped_files.len();
				if file_count == 1 {
					// Single file: @filename
					button_spans
						.push(Span::styled(format!("@{}", filename), Style::default().fg(self.theme.accent)));
				} else {
					// Multiple files: @filename+
					button_spans
						.push(Span::styled(format!("@{}+", filename), Style::default().fg(self.theme.accent)));
				}
			}

			// Add center dot separator if both file and paste are present
			if has_files && has_paste {
				button_spans.push(Span::styled(" · ", Style::default().fg(self.theme.accent)));
			}

			// Add paste text preview (if present)
			if has_paste {
				if let Some(ref clipboard_text) = self.clipboard_buffer {
					// Show first 30 chars of pasted text
					let preview = if clipboard_text.len() > 30 {
						format!("{}...", &clipboard_text[..30].replace('\n', " "))
					} else {
						clipboard_text.replace('\n', " ")
					};
					button_spans.push(Span::styled(preview, Style::default().fg(self.theme.accent)));
				}
			}

			// Calculate button areas for click detection
			let button_line = Line::from(button_spans.clone());
			let button_text_width = button_line.width() as u16;
			let center_x = center_area.x + (center_area.width.saturating_sub(button_text_width)) / 2;

			// Track button areas
			let mut current_x = center_x;

			if has_files {
				// Calculate width based on actual content
				let first_file = &self.dropped_files[0];
				let filename = std::path::Path::new(first_file)
					.file_name()
					.and_then(|n| n.to_str())
					.unwrap_or(first_file);

				let file_count = self.dropped_files.len();
				let file_width = if file_count == 1 {
					1 + filename.len() as u16 // "@" + filename
				} else {
					2 + filename.len() as u16 // "@" + filename + "+"
				};
				self.file_button_area =
					Rect { x: current_x, y: center_area.y, width: file_width, height: 1 };
				current_x += file_width;

				if has_paste {
					current_x += 3; // " · " separator
				}
			}

			if has_paste {
				if let Some(ref clipboard_text) = self.clipboard_buffer {
					let preview = if clipboard_text.len() > 30 {
						format!("{}...", &clipboard_text[..30].replace('\n', " "))
					} else {
						clipboard_text.replace('\n', " ")
					};
					let paste_width = preview.len() as u16;
					self.paste_button_area =
						Rect { x: current_x, y: center_area.y, width: paste_width, height: 1 };
				}
			}

			// Render the content
			Paragraph::new(button_line)
				.alignment(ratatui::layout::Alignment::Center)
				.render(center_area, buf);
		} else {
			// No buttons - show shortcuts carousel
			Paragraph::new(Span::styled(current_shortcut, Style::default().fg(self.theme.border)))
				.alignment(ratatui::layout::Alignment::Center)
				.render(center_area, buf);
		}

		// Token usage with color based on ratio
		let token_color = if token_ratio > 80 {
			ratatui::style::Color::Red
		} else if token_ratio > 60 {
			ratatui::style::Color::Yellow
		} else {
			self.theme.fg
		};

		Paragraph::new(Span::styled(&token_info, Style::default().fg(token_color)))
			.alignment(ratatui::layout::Alignment::Left)
			.render(bottom_chunks[6], buf);

		Paragraph::new(Span::styled(&truncated_path, Style::default().fg(self.theme.fg)))
			.alignment(ratatui::layout::Alignment::Left)
			.render(bottom_chunks[8], buf);

		// Store path button area for click detection
		self.path_button_area = bottom_chunks[8];

		// Only show spinner when loading
		if self.is_loading {
			let spinner_frames = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
			let elapsed_ms = (self.rainbow_animation.elapsed() * 1000.0) as u64;
			let frame_idx = ((elapsed_ms / 80) as usize) % spinner_frames.len();
			let spinner_char = spinner_frames[frame_idx];

			let color = self.rainbow_animation.rgb_color_at(frame_idx);
			let ratatui_color = ratatui::style::Color::Rgb(color.r, color.g, color.b);

			Paragraph::new(Span::styled(
				spinner_char.to_string(),
				Style::default().fg(ratatui_color).add_modifier(Modifier::BOLD),
			))
			.alignment(ratatui::layout::Alignment::Left)
			.render(bottom_chunks[10], buf);
		}

		(bottom_chunks[2], bottom_chunks[4], bottom_chunks[6], bottom_chunks[0])
	}

	pub fn render_perf_overlay(&self, area: Rect, buf: &mut Buffer) {
		if !self.show_perf_overlay {
			return;
		}

		let stats = self.perf_monitor.get_stats();

		// Create overlay area (top-right corner, 50 chars wide, 10 lines tall)
		let overlay_width = 52.min(area.width);
		let overlay_height = 10.min(area.height);
		let overlay_area = Rect {
			x: area.width.saturating_sub(overlay_width),
			y: 0,
			width: overlay_width,
			height: overlay_height,
		};

		// Determine status color
		let status_color = if self.perf_monitor.is_meeting_targets() {
			ratatui::style::Color::Green
		} else if stats.avg_frame_render_ms < 50.0 {
			ratatui::style::Color::Yellow
		} else {
			ratatui::style::Color::Red
		};

		// Build content lines
		let lines = vec![
			Line::from(vec![
				Span::styled("> ", Style::default().fg(ratatui::style::Color::Yellow)),
				Span::styled(
					"Performance Monitor",
					Style::default().fg(ratatui::style::Color::Cyan).add_modifier(Modifier::BOLD),
				),
			]),
			Line::from(""),
			Line::from(vec![
				Span::raw("Input:    "),
				Span::styled(
					format!("{:.2}ms", stats.avg_input_render_ms),
					Style::default().fg(if stats.avg_input_render_ms < 16.0 {
						ratatui::style::Color::Green
					} else {
						ratatui::style::Color::Yellow
					}),
				),
			]),
			Line::from(vec![
				Span::raw("Status:  "),
				Span::styled(
					if self.perf_monitor.is_meeting_targets() { "✓ EXCELLENT" } else { "○ GOOD" },
					Style::default().fg(status_color).add_modifier(Modifier::BOLD),
				),
			]),
		];

		let block = Block::default()
			.borders(Borders::ALL)
			.border_style(Style::default().fg(status_color))
			.border_type(ratatui::widgets::BorderType::Rounded)
			.style(Style::default().bg(ratatui::style::Color::Black));

		let paragraph = Paragraph::new(lines).block(block).style(Style::default().fg(self.theme.fg));

		paragraph.render(overlay_area, buf);
	}
}

impl ChatState {
	/// Render toast notification in top-right corner
	pub fn render_toast(&self, area: Rect, buf: &mut Buffer) {
		if let Some(ref message) = self.toast_message {
			// Toast dimensions
			let message_len: usize = message.len();
			let toast_width: u16 = (message_len as u16 + 4).min(area.width);
			let toast_height = 3;

			// Position in top-right corner
			let toast_x = area.width.saturating_sub(toast_width);
			let toast_y = 0;

			let toast_area = Rect { x: toast_x, y: toast_y, width: toast_width, height: toast_height };

			// Create toast with border
			let block = Block::default()
				.borders(Borders::ALL)
				.border_style(Style::default().fg(self.theme.accent))
				.border_type(ratatui::widgets::BorderType::Rounded)
				.style(Style::default().bg(self.theme.bg));

			let inner = block.inner(toast_area);
			block.render(toast_area, buf);

			// Render message text
			let text = Paragraph::new(message.as_str())
				.style(Style::default().fg(self.theme.fg))
				.alignment(ratatui::layout::Alignment::Center);

			text.render(inner, buf);
		}
	}

	/// Render intro/outro indicators in top-left corner (for carousel screens)
	pub fn render_animation_indicators(
		&self,
		area: Rect,
		current_anim: AnimationType,
		buf: &mut Buffer,
	) {
		// Only show on carousel animations (not Splash or Yazi)
		if current_anim == AnimationType::Splash || current_anim == AnimationType::Yazi {
			return;
		}

		let mut lines = Vec::new();

		// Show intro indicator
		if self.intro_animation == current_anim {
			lines.push(Line::from(vec![
				Span::styled("▲ ", Style::default().fg(self.theme.accent)),
				Span::styled("INTRO", Style::default().fg(self.theme.fg)),
			]));
		}

		// Show outro indicator
		if self.outro_animation == current_anim {
			lines.push(Line::from(vec![
				Span::styled("▼ ", Style::default().fg(self.theme.accent)),
				Span::styled("OUTRO", Style::default().fg(self.theme.fg)),
			]));
		}

		if lines.is_empty() {
			return;
		}

		// Calculate dimensions
		let indicator_height = lines.len() as u16 + 2; // +2 for border
		let indicator_width = 12; // Fixed width for "▼ OUTRO" + padding

		let indicator_area = Rect { x: 0, y: 0, width: indicator_width, height: indicator_height };

		// Create indicator box with border
		let block = Block::default()
			.borders(Borders::ALL)
			.border_style(Style::default().fg(self.theme.accent))
			.border_type(ratatui::widgets::BorderType::Rounded)
			.style(Style::default().bg(self.theme.bg));

		let inner = block.inner(indicator_area);
		block.render(indicator_area, buf);

		// Render indicator text
		let text = Paragraph::new(lines).style(Style::default().fg(self.theme.fg));

		text.render(inner, buf);
	}

	/// Render model picker overlay
	pub fn render_model_picker(&self, area: Rect, buf: &mut Buffer) {
		use crate::models::{ModelProvider, get_available_models};
		use ratatui::prelude::Stylize;
		use ratatui::style::{Modifier, Style};
		use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Widget};

		// Get all available models
		let models = get_available_models();

		// Calculate menu dimensions
		let menu_width = 60;
		let menu_height = (models.len() as u16 + 4).min(area.height.saturating_sub(4));

		// Center the menu
		let menu_x = area.width.saturating_sub(menu_width) / 2;
		let menu_y = area.height.saturating_sub(menu_height) / 2;

		let menu_area = Rect { x: menu_x, y: menu_y, width: menu_width, height: menu_height };

		// Create semi-transparent background
		for y in menu_area.y..menu_area.y + menu_area.height {
			for x in menu_area.x..menu_area.x + menu_area.width {
				if let Some(cell) = buf.cell_mut((x, y)) {
					cell.set_bg(self.theme.bg);
				}
			}
		}

		// Create border
		let block = Block::default()
			.title(" Select Model ")
			.title_style(Style::default().fg(self.theme.accent).add_modifier(Modifier::BOLD))
			.borders(Borders::ALL)
			.border_type(BorderType::Rounded)
			.border_style(Style::default().fg(self.theme.accent))
			.style(Style::default().bg(self.theme.bg));

		let inner = block.inner(menu_area);
		block.render(menu_area, buf);

		// Split into sections: Local and Codex
		let local_models: Vec<_> =
			models.iter().filter(|m| m.provider == ModelProvider::Local).collect();
		let codex_models: Vec<_> =
			models.iter().filter(|m| m.provider == ModelProvider::Codex).collect();

		// Create list items
		let mut items = Vec::new();

		// Local section
		items.push(ListItem::new(Span::styled(
			"▼ Local Models",
			Style::default().fg(self.theme.accent).add_modifier(Modifier::BOLD),
		)));

		for model in local_models {
			let is_selected = model.id == self.current_model.id;
			let prefix = if is_selected { "● " } else { "  " };
			let style = if is_selected {
				Style::default().fg(self.theme.accent).add_modifier(Modifier::BOLD)
			} else {
				Style::default().fg(self.theme.fg)
			};

			let display = if model.is_unlimited {
				format!("{}{} (Unlimited)", prefix, model.display_name)
			} else {
				format!("{}{}", prefix, model.display_name)
			};

			items.push(ListItem::new(Span::styled(display, style)));
		}

		// Codex section
		items.push(ListItem::new(""));
		items.push(ListItem::new(Span::styled(
			"▼ Codex Models",
			Style::default().fg(self.theme.accent).add_modifier(Modifier::BOLD),
		)));

		for model in codex_models {
			let is_selected = model.id == self.current_model.id;
			let prefix = if is_selected { "● " } else { "  " };
			let style = if is_selected {
				Style::default().fg(self.theme.accent).add_modifier(Modifier::BOLD)
			} else {
				Style::default().fg(self.theme.fg)
			};

			let context = if let Some(ctx) = model.context_window {
				format!(" ({}k)", ctx / 1000)
			} else {
				String::new()
			};

			let display = format!("{}{}{}", prefix, model.display_name, context);
			items.push(ListItem::new(Span::styled(display, style)));
		}

		// Render list
		let list = List::new(items).style(Style::default().bg(self.theme.bg));
		Widget::render(list, inner, buf);

		// Render hint at bottom
		let hint_y = menu_area.y + menu_area.height;
		if hint_y < area.height {
			let hint = Paragraph::new(Span::styled(
				"Click to select • ESC to close",
				Style::default().fg(self.theme.secondary).italic(),
			))
			.alignment(ratatui::layout::Alignment::Center);

			let hint_area = Rect { x: menu_area.x, y: hint_y, width: menu_area.width, height: 1 };

			hint.render(hint_area, buf);
		}
	}
}
