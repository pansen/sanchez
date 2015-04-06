use fern;
use log;
use time;

/// boilerplate code to setup logging with `fern`
pub fn setup_logging() {
	let logger_config = fern::DispatchConfig {
	    format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
	        // This is a fairly simple format, though it's possible to do more complicated ones.
	        // This closure can contain any code, as long as it produces a String message.
	        format!("[{}][{}] {}", time::now().strftime("%Y-%m-%d][%H:%M:%S").unwrap(), level, msg)
	    }),
	    // add a file-output: ``, fern::OutputConfig::file("output.log")``
	    output: vec![fern::OutputConfig::stdout()],
	    level: log::LogLevelFilter::Trace,
	};

	if let Err(e) = fern::init_global_logger(logger_config, log::LogLevelFilter::Trace) {
	    panic!("Failed to initialize global logger: {}", e);
	}
}

