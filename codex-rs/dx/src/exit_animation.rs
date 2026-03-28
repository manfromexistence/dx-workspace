use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use super::effects::RainbowEffect;

// Global state for tracking if animation is running
static ANIMATION_RUNNING: AtomicBool = AtomicBool::new(false);
static SHOULD_RESET: AtomicBool = AtomicBool::new(false);

pub fn show_train_farewell() {
	// If animation is already running, user pressed Ctrl+C again - reset it
	if ANIMATION_RUNNING.load(Ordering::SeqCst) {
		SHOULD_RESET.store(true, Ordering::SeqCst);
		// Wait a bit for the animation to detect the reset
		thread::sleep(Duration::from_millis(100));
		return;
	}

	ANIMATION_RUNNING.store(true, Ordering::SeqCst);
	SHOULD_RESET.store(false, Ordering::SeqCst);

	loop {
		let rainbow = RainbowEffect::new();

		// Clear the entire screen first
		print!("\x1B[2J"); // Clear screen
		print!("\x1B[H"); // Move cursor to home (top-left)
		let _ = io::stdout().flush();

		// Get terminal size
		let size = crossterm::terminal::size().unwrap_or((120, 30));
		let terminal_width = size.0 as usize;

		// Start audio playback in a separate thread
		let audio_handle = thread::spawn(|| {
			play_train_sounds();
		});

		// Show train animation for 40 frames (8 seconds at 200ms per frame) to match sound duration
		let mut reset_detected = false;
		for frame in 0..40 {
			// Check if reset was requested
			if SHOULD_RESET.load(Ordering::SeqCst) {
				SHOULD_RESET.store(false, Ordering::SeqCst);
				reset_detected = true;
				break;
			}

			// Move cursor to top for each frame
			print!("\x1B[H");

			render_train_frame(&rainbow, frame, terminal_width);

			thread::sleep(Duration::from_millis(200));
		}

		// If reset was detected, wait for audio to finish before restarting
		if reset_detected {
			// Wait for audio thread to complete to avoid audio device conflicts
			let _ = audio_handle.join();
			// Small delay to ensure audio device is fully released
			thread::sleep(Duration::from_millis(100));
			continue;
		}

		// Animation completed normally, wait for audio and exit
		let _ = audio_handle.join();
		break;
	}

	ANIMATION_RUNNING.store(false, Ordering::SeqCst);
	println!();
}

fn play_train_sounds() {
	use rodio::{Decoder, OutputStream, Sink, Source};
	use std::io::Cursor;

	// Embed train sounds directly into binary
	const TRAIN_WHISTLE_SOUND: &[u8] = include_bytes!("../assets/train-whistle.mp3");
	const TRAIN_RUNNING_SOUND: &[u8] = include_bytes!("../assets/train-running.mp3");

	// Retry logic for audio device initialization
	let mut stream_result = None;
	for attempt in 0..3 {
		if attempt > 0 {
			// Wait a bit before retrying
			thread::sleep(Duration::from_millis(100));
		}

		match OutputStream::try_default() {
			Ok(result) => {
				stream_result = Some(result);
				break;
			}
			Err(_) => {
				// Silently retry - don't show errors
			}
		}
	}

	let Some((_stream, stream_handle)) = stream_result else {
		// Silently fail if no audio device
		return;
	};

	let Ok(sink) = Sink::try_new(&stream_handle) else {
		// Silently fail if can't create sink
		return;
	};

	// Set volume to 5% (quieter train sounds)
	sink.set_volume(0.05);

	// Play whistle sound from embedded data
	let cursor = Cursor::new(TRAIN_WHISTLE_SOUND);
	if let Ok(source) = Decoder::new(cursor) {
		let buffered = source.buffered();
		sink.append(buffered);
	}

	// Play running sound from embedded data
	let cursor = Cursor::new(TRAIN_RUNNING_SOUND);
	if let Ok(source) = Decoder::new(cursor) {
		let buffered = source.buffered();
		sink.append(buffered);
	}

	// Wait for sounds to finish playing
	// This keeps the stream and sink alive until playback completes
	sink.sleep_until_end();

	// Give a small buffer to ensure audio finishes cleanly
	thread::sleep(Duration::from_millis(100));
}

fn render_train_frame(rainbow: &RainbowEffect, frame: usize, terminal_width: usize) {
	let elapsed_ms = frame * 200;
	let train_width = 55;

	// Train moves from right to left - slower to better match sound duration
	let total_travel = terminal_width + train_width + 10;
	let cycle_duration = 7500; // 7.5 seconds for one complete pass (slightly slower)
	let progress = (elapsed_ms as f32) / (cycle_duration as f32);
	let x_pos = (terminal_width as f32 + 10.0 - progress * total_travel as f32) as i32;

	let train = vec![
		"      ====        ________                ___________",
		"  _D _|  |_______/        \\__I_I_____===__|_________|",
		"   |(_)---  |   H\\________/ |   |        =|___ ___|",
		"   /     |  |   H  |  |     |   |         ||_| |_||",
		"  |      |  |   H  |__--------------------| [___] |",
		"  | ________|___H__/__|_____/[][]~\\_______|       |",
		"  |/ |   |-----------I_____I [][] []  D   |=======|",
		"__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|",
		" |/-=|___|=O=====O=====O=====O   |_____/~\\___/",
		"  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/",
	];

	// Smoke animation
	let smoke_frames: Vec<&[&str]> = vec![
		&["    (  )", "   (    )", "  (      )"],
		&["   (   )", "  (     )", " (       )"],
		&["  (    )", " (      )", "(        )"],
	];
	let smoke_frame_idx = (elapsed_ms / 300) % smoke_frames.len();
	let smoke = smoke_frames[smoke_frame_idx];

	// Render smoke
	let smoke_x_offset = x_pos + 6;
	for smoke_line in smoke {
		// Clear line and render
		for x in 0..terminal_width {
			if smoke_x_offset >= 0
				&& x >= smoke_x_offset as usize
				&& x < (smoke_x_offset as usize + smoke_line.len())
			{
				let char_idx = x - smoke_x_offset as usize;
				if let Some(ch) = smoke_line.chars().nth(char_idx) {
					let color_idx = (char_idx + (elapsed_ms / 200)) % 50;
					let c = rainbow.rgb_color_at(color_idx);
					print!("\x1B[38;2;{};{};{}m{}\x1B[0m", c.r, c.g, c.b, ch);
					continue;
				}
			} else if smoke_x_offset < 0 {
				let visible_start = (-smoke_x_offset) as usize;
				if x < smoke_line.len().saturating_sub(visible_start)
					&& let Some(ch) = smoke_line.chars().nth(x + visible_start)
				{
					let color_idx = (x + visible_start + (elapsed_ms / 200)) % 50;
					let c = rainbow.rgb_color_at(color_idx);
					print!("\x1B[38;2;{};{};{}m{}\x1B[0m", c.r, c.g, c.b, ch);
					continue;
				}
			}
			print!(" ");
		}
		println!();
	}

	// Render train
	for (line_idx, line) in train.iter().enumerate() {
		for x in 0..terminal_width {
			if x_pos >= 0 && x >= x_pos as usize && x < (x_pos as usize + line.len()) {
				let char_idx = x - x_pos as usize;
				if let Some(ch) = line.chars().nth(char_idx) {
					let color_idx = (char_idx + line_idx * 3 + (elapsed_ms / 150)) % 50;
					let c = rainbow.rgb_color_at(color_idx);
					print!("\x1B[38;2;{};{};{}m{}\x1B[0m", c.r, c.g, c.b, ch);
					continue;
				}
			} else if x_pos < 0 {
				let visible_start = (-x_pos) as usize;
				if x < line.len().saturating_sub(visible_start)
					&& let Some(ch) = line.chars().nth(x + visible_start)
				{
					let color_idx = (x + visible_start + line_idx * 3 + (elapsed_ms / 150)) % 50;
					let c = rainbow.rgb_color_at(color_idx);
					print!("\x1B[38;2;{};{};{}m{}\x1B[0m", c.r, c.g, c.b, ch);
					continue;
				}
			}
			print!(" ");
		}
		println!();
	}

	// Render tracks
	for x in 0..terminal_width {
		let ch = if (x + (elapsed_ms / 300)) % 4 == 0 { '╫' } else { '═' };
		let color_idx = (x + (elapsed_ms / 300)) % 50;
		let c = rainbow.rgb_color_at(color_idx);
		print!("\x1B[38;2;{};{};{}m{}\x1B[0m", c.r, c.g, c.b, ch);
	}
	println!();

	let _ = io::stdout().flush();
}
