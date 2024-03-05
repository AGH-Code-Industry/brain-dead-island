use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

pub fn init_logging() {
    TermLogger::init(
        log::LevelFilter::Info,
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )
    .or_else(|err| {
        eprintln!("Failed to initialize logging system: {}", err);
        Ok::<(), ()>(())
    })
    .unwrap();
}
