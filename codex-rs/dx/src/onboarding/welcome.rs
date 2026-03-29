use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;
use crossterm::event::KeyModifiers;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::Clear;
use ratatui::widgets::Paragraph;
use ratatui::widgets::WidgetRef;
use ratatui::widgets::Wrap;
use std::cell::Cell;

use crate::ascii_animation::AsciiAnimation;
use crate::onboarding::onboarding_screen::KeyboardHandler;
use crate::onboarding::onboarding_screen::StepStateProvider;
use crate::tui::FrameRequester;

// DX-TUI integration
use crate::effects::RainbowEffect;
use crate::splash;
use crate::state::AnimationType;
use crate::theme::{ChatTheme, ThemeVariant};

use super::onboarding_screen::StepState;

const MIN_ANIMATION_HEIGHT: u16 = 37;
const MIN_ANIMATION_WIDTH: u16 = 60;

pub(crate) struct WelcomeWidget {
	pub is_logged_in: bool,
	animation: AsciiAnimation,
	animations_enabled: bool,
	layout_area: Cell<Option<Rect>>,
	// DX-TUI state
	dx_theme: ChatTheme,
	current_animation_index: usize,
	rainbow_effect: RainbowEffect,
	splash_font_index: usize,
}

impl KeyboardHandler for WelcomeWidget {
	fn handle_key_event(&mut self, key_event: KeyEvent) {
		if !self.animations_enabled {
			return;
		}
		if key_event.kind == KeyEventKind::Press
			&& key_event.code == KeyCode::Char('.')
			&& key_event.modifiers.contains(KeyModifiers::CONTROL)
		{
			tracing::warn!("Welcome background to press '.'");
			let _ = self.animation.pick_random_variant();
			
			// Cycle through DX splash fonts
			self.splash_font_index = (self.splash_font_index + 1) % 10;
			
			// Also cycle through DX animations
			let animations = AnimationType::all();
			self.current_animation_index = (self.current_animation_index + 1) % animations.len();
		}
	}
}

impl WelcomeWidget {
	pub(crate) fn new(
		is_logged_in: bool,
		request_frame: FrameRequester,
		animations_enabled: bool,
	) -> Self {
		Self {
			is_logged_in,
			animation: AsciiAnimation::new(request_frame),
			animations_enabled,
			layout_area: Cell::new(None),
			dx_theme: ChatTheme::new(ThemeVariant::Dark),
			current_animation_index: 0,
			rainbow_effect: RainbowEffect::new(),
			splash_font_index: 0,
		}
	}

	pub(crate) fn update_layout_area(&self, area: Rect) {
		self.layout_area.set(Some(area));
	}
}

impl WidgetRef for &WelcomeWidget {
	fn render_ref(&self, area: Rect, buf: &mut Buffer) {
		// Clear with DX theme background
		for y in area.top()..area.bottom() {
			for x in area.left()..area.right() {
				buf[(x, y)].reset();
				buf[(x, y)].set_bg(self.dx_theme.bg);
			}
		}
		
		// Always schedule next frame for rainbow animation
		if self.animations_enabled {
			self.animation.schedule_next_frame();
		}

		// Render DX-TUI splash screen instead of codex welcome
		splash::render(area, buf, &self.dx_theme, self.splash_font_index, &self.rainbow_effect);
		
		// Add hint about DX features at the bottom
		let hint_y = area.bottom().saturating_sub(2);
		if hint_y > area.y {
			let hint_area = Rect {
				x: area.x,
				y: hint_y,
				width: area.width,
				height: 2,
			};
			
			let hint_lines = vec![
				Line::from(vec![
					"  Press ".fg(self.dx_theme.muted_fg).into(),
					"Ctrl+.".fg(self.dx_theme.accent).bold(),
					" to cycle splash fonts".fg(self.dx_theme.muted_fg).into(),
				]),
			];
			
			Paragraph::new(hint_lines)
				.alignment(ratatui::layout::Alignment::Center)
				.render(hint_area, buf);
		}
	}
}

impl StepStateProvider for WelcomeWidget {
	fn get_step_state(&self) -> StepState {
		match self.is_logged_in {
			true => StepState::Hidden,
			false => StepState::Complete,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	use ratatui::buffer::Buffer;
	use ratatui::layout::Rect;

	static VARIANT_A: [&str; 1] = ["frame-a"];
	static VARIANT_B: [&str; 1] = ["frame-b"];
	static VARIANTS: [&[&str]; 2] = [&VARIANT_A, &VARIANT_B];

	fn row_containing(buf: &Buffer, needle: &str) -> Option<u16> {
		(0..buf.area.height).find(|&y| {
			let mut row = String::new();
			for x in 0..buf.area.width {
				row.push_str(buf[(x, y)].symbol());
			}
			row.contains(needle)
		})
	}

	#[test]
	fn welcome_renders_animation_on_first_draw() {
		let widget = WelcomeWidget::new(false, FrameRequester::test_dummy(), true);
		let area = Rect::new(0, 0, MIN_ANIMATION_WIDTH, MIN_ANIMATION_HEIGHT);
		let mut buf = Buffer::empty(area);
		let frame_lines = widget.animation.current_frame().lines().count() as u16;
		(&widget).render(area, &mut buf);

		let welcome_row = row_containing(&buf, "Welcome");
		assert_eq!(welcome_row, Some(frame_lines + 1));
	}

	#[test]
	fn welcome_skips_animation_below_height_breakpoint() {
		let widget = WelcomeWidget::new(false, FrameRequester::test_dummy(), true);
		let area = Rect::new(0, 0, MIN_ANIMATION_WIDTH, MIN_ANIMATION_HEIGHT - 1);
		let mut buf = Buffer::empty(area);
		(&widget).render(area, &mut buf);

		let welcome_row = row_containing(&buf, "Welcome");
		assert_eq!(welcome_row, Some(0));
	}

	#[test]
	fn ctrl_dot_changes_animation_variant() {
		let mut widget = WelcomeWidget {
			is_logged_in: false,
			animation: AsciiAnimation::with_variants(FrameRequester::test_dummy(), &VARIANTS, 0),
			animations_enabled: true,
			layout_area: Cell::new(None),
		};

		let before = widget.animation.current_frame();
		widget.handle_key_event(KeyEvent::new(KeyCode::Char('.'), KeyModifiers::CONTROL));
		let after = widget.animation.current_frame();

		assert_ne!(before, after, "expected ctrl+. to switch welcome animation variant");
	}
}
