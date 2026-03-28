// Audio playback module for DX animations
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub struct AudioPlayer {
	_stream: OutputStream,
	sink: Arc<Mutex<Option<Sink>>>,
}

impl AudioPlayer {
	pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
		let (stream, stream_handle) = OutputStream::try_default()?;
		let sink = Sink::try_new(&stream_handle)?;

		Ok(Self { _stream: stream, sink: Arc::new(Mutex::new(Some(sink))) })
	}

	/// Resolve the asset path relative to the executable or current directory
	fn resolve_asset_path(path: &str) -> PathBuf {
		// Try multiple locations
		let locations = vec![
			PathBuf::from(path),                                                     // Relative to current dir
			std::env::current_exe().ok().and_then(|exe| exe.parent().map(|p| p.join(path))), // Relative to exe
			std::env::current_dir().ok().map(|d| d.join(path)),                      // Explicit current dir
		];

		for location in locations.into_iter().flatten() {
			if location.exists() {
				return location;
			}
		}

		// Fallback to original path
		PathBuf::from(path)
	}

	/// Play a sound file (looping)
	pub fn play_looping(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
		// Stop any currently playing sound
		self.stop();

		// Resolve the full path
		let full_path = Self::resolve_asset_path(path);

		// Open the audio file - silently fail if not found
		let file = File::open(&full_path)?;
		let source = Decoder::new(BufReader::new(file))?;

		// Create a new sink and play
		if let Ok(mut sink_guard) = self.sink.lock() {
			if let Some(sink) = sink_guard.as_ref() {
				sink.append(source.repeat_infinite());
				sink.play();
			}
		}

		Ok(())
	}

	/// Stop the currently playing sound
	pub fn stop(&self) {
		if let Ok(sink_guard) = self.sink.lock() {
			if let Some(sink) = sink_guard.as_ref() {
				sink.stop();
			}
		}
	}

	/// Set volume (0.0 to 1.0)
	pub fn set_volume(&self, volume: f32) {
		if let Ok(sink_guard) = self.sink.lock() {
			if let Some(sink) = sink_guard.as_ref() {
				sink.set_volume(volume);
			}
		}
	}
}

impl Default for AudioPlayer {
	fn default() -> Self {
		Self::new().unwrap_or_else(|_| {
			// Fallback: create a dummy player if audio initialization fails
			let (stream, _) = OutputStream::try_default().unwrap();
			Self { _stream: stream, sink: Arc::new(Mutex::new(None)) }
		})
	}
}
