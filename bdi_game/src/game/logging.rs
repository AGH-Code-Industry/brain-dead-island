use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};

pub(crate) fn init_logging() {
    // Create log directory if it doesn't exist
    match std::fs::create_dir("logs") {
        Ok(_) => (),
        Err(e) => {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                eprintln!("Error while creating log directory: {}", e);
                std::process::exit(1);
            }
        }
    }

    let current_time_string = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let log_file_name = format!("logs/{}.log", current_time_string);
    let log_file = match std::fs::File::create(log_file_name.clone()) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error while creating log file: {}", e);
            std::process::exit(1);
        }
    };

    match CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(LevelFilter::Debug, Config::default(), log_file),
    ]) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error while initializing logger: {}", e);
            std::process::exit(1);
        }
    }
}
