// Audio playback module for DX animations
use rodio::{Decoder, OutputStream, Sink, Source};
use std::io::Cursor;
use std::sync::{Arc, Mutex};

// Embed audio files directly into the binary
const MATRIX_SOUND: &[u8] = include_bytes!("../assets/matrix.mp3");
const RAIN_SOUND: &[u8] = include_bytes!("../assets/rain.mp3");
const WAVE_SOUND: &[u8] = include_bytes!("../assets/wave.mp3");
const FIREWORKS_SOUND: &[u8] = include_bytes!("../assets/fireworks.mp3");
const SPACE_SOUND: &[u8] = include_bytes!("../assets/space.mp3");
const PLASMA_SOUND: &[u8] = include_bytes!("../assets/plasma.mp3");
const TRAIN_RUNNING_SOUND: &[u8] = include_bytes!("../assets/train-running.mp3");
const TRAIN_WHISTLE_SOUND: &[u8] = include_bytes!("../assets/train-whistle.mp3");
const CONFETTI_SOUND: &[u8] = include_bytes!("../assets/confetti.mp3");
const GAME_OF_LIFE_SOUND: &[u8] = include_bytes!("../assets/game-of-life.mp3");
const JUMP_SOUND: &[u8] = include_bytes!("../assets/jump.mp3");
const NEON_CAT_SOUND: &[u8] = include_bytes!("../assets/neon-cat.mp3");
const SOIL_SOUND: &[u8] = include_bytes!("../assets/soil.mp3");
const FIRE_SOUND: &[u8] = include_bytes!("../assets/fire.mp3");
const EAGLE_SOUND: &[u8] = include_bytes!("../assets/eagle.mp3");
const BIRDS_SOUND: &[u8] = include_bytes!("../assets/birds.mp3");

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

	/// Get embedded audio data by path
	fn get_embedded_audio(path: &str) -> Option<&'static [u8]> {
		match path {
			"assets/matrix.mp3" => Some(MATRIX_SOUND),
			"assets/rain.mp3" => Some(RAIN_SOUND),
			"assets/wave.mp3" => Some(WAVE_SOUND),
			"assets/fireworks.mp3" => Some(FIREWORKS_SOUND),
			"assets/space.mp3" => Some(SPACE_SOUND),
			"assets/plasma.mp3" => Some(PLASMA_SOUND),
			"assets/train-running.mp3" => Some(TRAIN_RUNNING_SOUND),
			"assets/train-whistle.mp3" => Some(TRAIN_WHISTLE_SOUND),
			"assets/confetti.mp3" => Some(CONFETTI_SOUND),
			"assets/game-of-life.mp3" => Some(GAME_OF_LIFE_SOUND),
			"assets/jump.mp3" => Some(JUMP_SOUND),
			"assets/neon-cat.mp3" => Some(NEON_CAT_SOUND),
			"assets/soil.mp3" => Some(SOIL_SOUND),
			"assets/fire.mp3" => Some(FIRE_SOUND),
			"assets/eagle.mp3" => Some(EAGLE_SOUND),
			"assets/birds.mp3" => Some(BIRDS_SOUND),
			_ => None,
		}
	}

	/// Play a sound file (looping) from embedded data
	pub fn play_looping(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
		// Stop any currently playing sound
		self.stop();

		// Get embedded audio data
		let audio_data = Self::get_embedded_audio(path).ok_or("Audio file not found")?;

		// Create a cursor from the embedded data
		let cursor = Cursor::new(audio_data);
		let source = Decoder::new(cursor)?;

		// Create a new sink and play
		if let Ok(sink_guard) = self.sink.lock() {
			if let Some(sink) = sink_guard.as_ref() {
				sink.append(source.repeat_infinite());
				sink.play();
			}
		}

		Ok(())
	}

	/// Play a sound file once (no looping) from embedded data
	pub fn play_once(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
		// Stop any currently playing sound
		self.stop();

		// Get embedded audio data
		let audio_data = Self::get_embedded_audio(path).ok_or("Audio file not found")?;

		// Create a cursor from the embedded data
		let cursor = Cursor::new(audio_data);
		let source = Decoder::new(cursor)?;

		// Create a new sink and play
		if let Ok(sink_guard) = self.sink.lock() {
			if let Some(sink) = sink_guard.as_ref() {
				sink.append(source);
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
