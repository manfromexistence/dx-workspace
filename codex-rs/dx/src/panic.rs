use std::backtrace::Backtrace;
use std::fs::{File, OpenOptions, create_dir_all};
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
			hook(info);
		}));
	}
}

pub(crate) fn write_panic_report(info: &std::panic::PanicHookInfo<'_>) {
	let Some(path) = panic_report_paths().into_iter().find_map(|path| {
		path.parent()
			.and_then(|dir| create_dir_all(dir).ok().map(|_| path.clone()))
			.and_then(|path| {
				OpenOptions::new().create(true).append(true).open(&path).ok().map(|_| path)
			})
	}) else {
		return;
	};
	let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&path) else {
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
	let _ = file.flush();
}

fn panic_report_paths() -> Vec<PathBuf> {
	let timestamp = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap_or_default()
		.as_millis();
	let filename = format!("panic-{timestamp}.log");
	let mut paths = Vec::new();

	if let Ok(cwd) = std::env::current_dir() {
		paths.push(cwd.join(".dx").join("tui").join("panic-reports").join(&filename));
	}

	paths.push(
		std::env::temp_dir()
			.join("codex-tui-dx")
			.join("panic-reports")
			.join(&filename),
	);

	paths
}
