use fern;
use log::{LevelFilter, debug};
use amethyst::{LoggerConfig, StdoutLog};

/// Hides the "Created buffer" annoying message
/// Shamefully copy-pasted from Amethyst code.
// TODO: Post an Issue and remove this
pub fn start_custom_logger(config: LoggerConfig) {
//    if config.allow_env_override {
//        env_var_override(&mut config);
//    }

    let mut dispatch =
        fern::Dispatch::new()
            .level(config.level_filter)
            .level_for("gfx_device_gl", LevelFilter::Warn)
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{level}][{target}] {message}",
                    level = record.level(),
                    target = record.target(),
                    message = message,
                ))
            });

    match config.stdout {
        StdoutLog::Plain => dispatch = dispatch.chain(std::io::stdout()),
        StdoutLog::Colored => dispatch = dispatch.chain({
            let color_config = fern::colors::ColoredLevelConfig::new();

            fern::Dispatch::new()
                .chain(std::io::stdout())
                .format(move |out, message, record| {
                    let color = color_config.get_color(&record.level());
                    out.finish(format_args!(
                        "{color}{message}{color_reset}",
                        color = format!("\x1B[{}m", color.to_fg_str()),
                        message = message,
                        color_reset = "\x1B[0m",
                    ))
                })
        }),
        StdoutLog::Off => {}
    }

    if let Some(path) = config.log_file {
        match fern::log_file(path) {
            Ok(log_file) => dispatch = dispatch.chain(log_file),
            Err(_) => eprintln!("Unable to access the log file, as such it will not be used"),
        }
    }

    dispatch.apply().unwrap_or_else(|_| {
        debug!("Global logger already set, default Amethyst logger will not be used")
    });
}
