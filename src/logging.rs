use fern;
use log;
use time;
use arguments::AppConfig;

#[allow(unused_assignments)]
/// boilerplate code to setup logging with `fern`
pub fn setup_logging(config: &AppConfig) {
    let mut log_level = log::LogLevelFilter::Off;

    match config.verbose {
        0 => log_level = log::LogLevelFilter::Error,
        1 => log_level = log::LogLevelFilter::Warn,
        2 => log_level = log::LogLevelFilter::Info,
        3 => log_level = log::LogLevelFilter::Debug,
        _ => log_level = log::LogLevelFilter::Trace
    }

    let logger_config = fern::DispatchConfig {
        format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
            // This is a fairly simple format, though it's possible to do more complicated ones.
            // This closure can contain any code, as long as it produces a String message.
            //format!("[{}][{}] {}", time::now().strftime("%Y-%m-%d][%H:%M:%S").unwrap(), level, msg)
            format!("[{}][{}] {}", time::now().strftime("%H:%M:%S").unwrap(), level, msg)
        }),
        // add a file-output: ``, fern::OutputConfig::file("output.log")``
        output: vec![fern::OutputConfig::stdout()],
        level: log_level,
    };

    if let Err(e) = fern::init_global_logger(logger_config, log_level) {
        panic!("Failed to initialize global logger: {}", e);
    }
}

