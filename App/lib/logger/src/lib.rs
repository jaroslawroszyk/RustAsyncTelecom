use anyhow::Result;
use dotenv_codegen::dotenv;
pub use log;
pub use log::Level;
use simplelog::*;
use std::fmt;
use std::panic::Location;
use std::thread;

const WORKDIR: &str = dotenv!("WORKDIR");
const TERM_FILTER: &str = dotenv!("TERM_LOG_LEVEL_FILTER");
const WRITE_FILTER: &str = dotenv!("WRITE_LOG_LEVEL_FILTER");

thread_local! {
    static THREAD_HEX_ID: String = {
        let id_str = format!("{:?}", thread::current().id());
        let id_num: u64 = id_str.chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse()
            .unwrap_or(0);
        format!("{:04x}", id_num)
    };
}

pub fn init() -> Result<()> {
    let date = chrono::Local::now().format("%Y%m%d%H");
    let path = format!("{WORKDIR}/artifacts/{date}.log");

    std::fs::create_dir_all(format!("{WORKDIR}/artifacts"))?;

    let file = std::fs::File::options()
        .append(true)
        .create(true)
        .open(path)?;

    let config = ConfigBuilder::new()
        .set_level_padding(LevelPadding::Right)
        .set_thread_level(LevelFilter::Off)
        .set_location_level(LevelFilter::Off)
        .set_target_level(LevelFilter::Off)
        .build();

    CombinedLogger::init(vec![
        WriteLogger::new(parse_level(WRITE_FILTER), config.clone(), file),
        TermLogger::new(
            parse_level(TERM_FILTER),
            config,
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
    ])?;

    Ok(())
}

fn parse_level(level: &str) -> LevelFilter {
    match level.parse::<u8>().unwrap_or(1) {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    }
}

#[doc(hidden)]
pub fn log_internal(level: log::Level, loc: &Location, args: fmt::Arguments) {
    if level <= log::max_level() {
        THREAD_HEX_ID.with(|id| {
            let file_path = loc.file();
            let file_name = file_path.rsplit('/').next().unwrap_or(file_path);

            match level {
                log::Level::Error => log::error!("({id}) {args}"),
                log::Level::Warn => log::warn!("({id}) {args}"),
                log::Level::Info => log::info!("({id}) {args}"),
                log::Level::Debug => log::debug!("({id}) [{file_name}:{}] {args}", loc.line()),
                log::Level::Trace => log::trace!("({id}) [{loc}] {args}"),
            }
        });
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => {
        $crate::log_internal(
            $crate::log::Level::Info,
            std::panic::Location::caller(),
            format_args!($($arg)+)
        )
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => {
        $crate::log_internal($crate::log::Level::Error, std::panic::Location::caller(), format_args!($($arg)+))
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => {
        $crate::log_internal($crate::log::Level::Warn, std::panic::Location::caller(), format_args!($($arg)+))
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => {
        $crate::log_internal($crate::log::Level::Debug, std::panic::Location::caller(), format_args!($($arg)+))
    };
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)+) => {
        $crate::log_internal($crate::log::Level::Trace, std::panic::Location::caller(), format_args!($($arg)+))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_level() {
        assert_eq!(parse_level("1"), LevelFilter::Error);
        assert_eq!(parse_level("3"), LevelFilter::Info);
        assert_eq!(parse_level("invalid"), LevelFilter::Error);
        assert_eq!(parse_level("10"), LevelFilter::Trace);
    }

    #[test]
    fn test_thread_id_consistency() {
        let id1 = THREAD_HEX_ID.with(|id| id.clone());
        let id2 = THREAD_HEX_ID.with(|id| id.clone());
        assert_eq!(id1, id2, "Thread ID must be stable within the same thread");
    }
}
