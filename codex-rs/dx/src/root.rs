use fb_binding::elements::render_once;
use fb_core::Core;
use fb_plugin::LUA;
use mlua::{ObjectLike, Table};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use tracing::error;
use fb_shared::url::AsUrl; // For as_url() method

use crate::file_browser::{cmp, confirm, help, input, mgr, pick, spot, tasks, which};
use crate::{
	bridge::YaziChatBridge,
	state::{AnimationType, ChatState},
};

pub struct Root<'a> {
	core: &'a Core,
	bridge: &'a mut YaziChatBridge,
	chat_state: &'a ChatState, // DIRECT reference to the REAL ChatState from ChatWidget
}

impl<'a> Root<'a> {
	pub fn new(core: &'a Core, bridge: &'a mut YaziChatBridge, chat_state: &'a ChatState) -> Self {
		Self { core, bridge, chat_state }
	}

	// For DX binary only - uses bridge's chat_state
	pub fn new_from_bridge(core: &'a Core, bridge: &'a mut YaziChatBridge) -> Self {
		// SAFETY: We're creating a reference to bridge.chat_state that lives as long as bridge
		// This is safe because bridge is borrowed mutably for 'a
		let chat_state_ptr = &bridge.chat_state as *const ChatState;
		let chat_state = unsafe { &*chat_state_ptr };
		Self { core, bridge, chat_state }
	}

	pub fn reflow(area: Rect) -> mlua::Result<Table> {
		let area = fb_binding::elements::Rect::from(area);
		let root = LUA.globals().raw_get::<Table>("Root")?.call_method::<Table>("new", area)?;
		root.call_method("reflow", ())
	}
}

impl Widget for Root<'_> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		// Clear the entire screen with theme background color first
		let bg_color = self.chat_state.theme.bg;
		for y in area.top()..area.bottom() {
			for x in area.left()..area.right() {
				buf[(x, y)].reset();
				buf[(x, y)].set_bg(bg_color);
			}
		}

		// PRIORITY 1: Check if we're in animation mode (splash/animations carousel)
		if self.chat_state.animation_mode {
			let animations = AnimationType::all();
			let current_anim = animations[self.chat_state.current_animation_index];

			// For Matrix animation, clear everything first before any rendering
			if current_anim == AnimationType::Matrix {
				let bg_color = self.chat_state.theme_bg_color();
				for y in area.top()..area.bottom() {
					for x in area.left()..area.right() {
						buf[(x, y)].reset();
						buf[(x, y)].set_bg(bg_color);
					}
				}
			}

			// Special case: Yazi screen in animation carousel
			if current_anim == AnimationType::Yazi {
				// Show Yazi file picker FULLSCREEN (no chat input - codex bottom pane handles that)
				let yazi_area = area;

				// Render yazi fullscreen using REAL DX CODE
				let mut f = || {
					let area = fb_binding::elements::Rect::from(yazi_area);
					let root = LUA.globals().raw_get::<Table>("Root")?.call_method::<Table>("new", area)?;

					render_once(root.call_method("redraw", ())?, buf, |p| self.core.mgr.area(p));
					Ok::<_, mlua::Error>(())
				};
				if let Err(e) = f() {
					error!("Failed to redraw the `Root` component:\n{e}");
				}

				mgr::Preview::new(self.core).render(yazi_area, buf);
				mgr::Modal::new(self.core).render(yazi_area, buf);

				if self.core.tasks.visible {
					tasks::Tasks::new(self.core).render(yazi_area, buf);
				}

				if self.core.active().spot.visible() {
					spot::Spot::new(self.core).render(yazi_area, buf);
				}

				if self.core.pick.visible {
					pick::Pick::new(self.core).render(yazi_area, buf);
				}

				if self.core.confirm.visible {
					confirm::Confirm::new(self.core).render(yazi_area, buf);
				}

				if self.core.help.visible {
					help::Help::new(self.core).render(yazi_area, buf);
				}

				if self.core.cmp.visible {
					cmp::Cmp::new(self.core).render(yazi_area, buf);
				}

				if self.core.which.active {
					which::Which::new(self.core).render(yazi_area, buf);
				}

				return;
			}

			// All other animations - render ONLY the animation (no DX chat UI)
			match current_anim {
				AnimationType::Splash => {
					crate::splash::render(
						area,
						buf,
						&self.chat_state.theme,
						self.chat_state.splash_font_index,
						&self.chat_state.rainbow_animation,
					);
				}
				AnimationType::Matrix => {
					self.chat_state.render_matrix_animation_in_area(area, buf);
				}
				AnimationType::Confetti => {
					self.chat_state.render_confetti_animation_in_area(area, buf);
				}
				AnimationType::GameOfLife => {
					self.chat_state.render_gameoflife_animation_in_area(area, buf);
				}
				AnimationType::Starfield => {
					self.chat_state.render_starfield_animation_in_area(area, buf);
				}
				AnimationType::Rain => {
					self.chat_state.render_rain_animation_in_area(area, buf);
				}
				AnimationType::NyanCat => {
					self.chat_state.render_nyancat_animation_in_area(area, buf);
				}
				AnimationType::DVDLogo => {
					self.chat_state.render_dvdlogo_animation_in_area(area, buf);
				}
				AnimationType::Fire => {
					self.chat_state.render_fire_animation_in_area(area, buf);
				}
				AnimationType::Plasma => {
					self.chat_state.render_plasma_animation_in_area(area, buf);
				}
				AnimationType::Waves => {
					self.chat_state.render_waves_animation_in_area(area, buf);
				}
				AnimationType::Fireworks => {
					self.chat_state.render_fireworks_animation_in_area(area, buf);
				}
				_ => {}
			}
			return;
		}

		// PRIORITY 2: Normal operation - don't render anything (ChatWidget handles it)
		// Root widget is ONLY for animations and Yazi
	}
}
