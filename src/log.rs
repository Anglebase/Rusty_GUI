use std::sync::Mutex;

use colored::Colorize;

/// Log levels
#[allow(unused)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
    Fatal = 4,
}

static mut LOG_LEVEL: Mutex<LogLevel> = Mutex::new(LogLevel::Info);

/// Output a debug message
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        log(LogLevel::Debug, &format!($($arg)*))
    };
}
/// Output an info message
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        log(LogLevel::Info, &format!($($arg)*))
    };
}
/// Output a warning message
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        log(LogLevel::Warn, &format!($($arg)*))
    };
}
/// Output an error message
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        log(LogLevel::Error, &format!($($arg)*))
    };
}
/// Output a fatal message.
#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {
        log(LogLevel::Fatal, &format!($($arg)*))
    };
}

/// Set the log level.
#[allow(unused)]
pub fn set_log_level(level: LogLevel) {
    unsafe {
        *LOG_LEVEL.lock().unwrap() = level;
    }
    debug!("Log level set to {:?}", level);
}
/// Get the current log level.
#[allow(unused)]
pub fn get_log_level() -> LogLevel {
    unsafe { *LOG_LEVEL.lock().unwrap() }
}

/// Log a message.
/// It is used by the macros `debug!`, `info!`, `warn!`, `error!`, and `fatal!`.
#[allow(unused)]
pub fn log(level: LogLevel, message: &str) {
    let time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    match level {
        LogLevel::Debug if get_log_level() as i32 <= LogLevel::Debug as i32 => {
            let s = format!("{}| [DEBUG] {}", time, message);
            println!("{}", s.blue())
        }
        LogLevel::Info if get_log_level() as i32 <= LogLevel::Info as i32 => {
            let s = format!("{}| [INFO] {}", time, message);
            println!("{}", s.green())
        }
        LogLevel::Warn if get_log_level() as i32 <= LogLevel::Warn as i32 => {
            let s = format!("{}| [WARN] {}", time, message);
            println!("{}", s.yellow())
        }
        LogLevel::Error if get_log_level() as i32 <= LogLevel::Error as i32 => {
            let s = format!("{}| [ERROR] {}", time, message);
            eprintln!("{}", s.red())
        }
        LogLevel::Fatal if get_log_level() as i32 <= LogLevel::Fatal as i32 => {
            let s = format!("{}| [FATAL] {}", time, message);
            eprintln!("{}", s.red().bold().underline())
        }
        _ => {}
    }
}
