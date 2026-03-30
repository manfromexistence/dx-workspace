use fb_term::Term;
use std::backtrace::Backtrace;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Panic;

impl Panic {
	pub fn install() {
		better_panic::install();

		let hook = std::panic::take_hook();
		std::panic::set_hook(Box::new(move |info| {
			write_panic_report(info);
			Term::goodbye(|| {
				hook(info);
				1
			});
		}));
	}
}

fn write_panic_report(info: &std::panic::PanicHookInfo<'_>) {
	let Ok(path) = panic_report_path() else {
		return;
	};
	let Ok(mut file) = File::create(&path) else {
		return;
	};

	let location = info
		.location()
		.map(|loc| format!("{}:{}:{}", loc.file(), loc.line(), loc.column()))
		.unwrap_or_else(|| "<unknown>".to_string());
	let payload = if let Some(s) = info.payload().downcast_ref::<&str>() {
		(*s).to_string()
	} else if let Some(s) = info.payload().downcast_ref::<String>() {
		s.clone()
	} else {
		"<non-string panic payload>".to_string()
	};
	let backtrace = Backtrace::force_capture();

	let _ = writeln!(file, "codex-tui-dx panic report");
	let _ = writeln!(file, "location: {location}");
	let _ = writeln!(file, "payload: {payload}");
	let _ = writeln!(file);
	let _ = writeln!(file, "backtrace:\n{backtrace}");
}

fn panic_report_path() -> std::io::Result<PathBuf> {
	let base_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
	let panic_dir = base_dir.join(".dx").join("tui").join("panic-reports");
	create_dir_all(&panic_dir)?;
	let timestamp = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap_or_default()
		.as_secs();
	Ok(panic_dir.join(format!("panic-{timestamp}.log")))
}
