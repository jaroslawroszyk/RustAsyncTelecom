use anyhow::Ok;
use dotenv_codegen::dotenv;
use simplelog::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::panic::Location;

const WORKDIR: &str = dotenv!("WORKDIR2");
const TERM_FILTER: &str = dotenv!("TERM_LOG_LEVEL_FILTER");
const WRITE_FILTER: &str = dotenv!("WRITE_LOG_LEVEL_FILTER");

pub fn init() -> anyhow::Result<()> {
    let date = chrono::Local::now().format("%Y%m%d%H");
    let path = format!("{WORKDIR}/artifacts/{date}.log");

    std::fs::create_dir_all(format!("{WORKDIR}/artifacts"))?;

    let file = std::fs::File::options()
        .append(true)
        .create(true)
        .open(path)?;

    let config = ConfigBuilder::new()
        .set_level_padding(LevelPadding::Right)
        .set_location_level(LevelFilter::Off)
        .set_target_level(LevelFilter::Off)
        .set_thread_level(LevelFilter::Off)
        .build();

    CombinedLogger::init(vec![
        WriteLogger::new(level_filter(WRITE_FILTER), config.clone(), file),
        TermLogger::new(
            level_filter(TERM_FILTER),
            config,
            TerminalMode::Mixed,
            ColorChoice::Always,
        ),
    ])?;
    Ok(())
}

#[doc(hidden)]
pub fn log(level: usize, loc: &Location, content: String) {
    let mut s = DefaultHasher::new();
    (std::process::id(), std::thread::current().id()).hash(&mut s);
    let thread = format!("{:08x}", (s.finish() % 4294967295) as u32);

    let file_path = loc.file();
    let file_name = file_path.rsplit('/').next().unwrap_or(file_path);
    let module_name = if file_path.contains("client") {
        "[client]"
    } else if file_path.contains("server") {
        "[server]"
    } else {
        "[log]"
    };

    match level {
        1 => log::error!("({thread}) {content}"),
        2 => log::warn!("({thread}) {content}"),
        3 => log::info!("({thread}) {content}"),
        4 => log::debug!(
            "({thread}) {} [{file_name}:{}:{}] {content}",
            module_name,
            loc.line(),
            loc.column()
        ),
        5 => log::trace!("({thread}) [{module_name}] [{loc}] {content}"),
        _ => (),
    }
}

fn level_filter(level: &str) -> LevelFilter {
    let n: usize = level.parse().unwrap_or(1) % 6;
    unsafe { std::mem::transmute(n) }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => {{
        let loc = std::panic::Location::caller();
        $crate::log(1, loc, format!($($arg)+))
    }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => {{
        let loc = std::panic::Location::caller();
        $crate::log(2, loc, format!($($arg)+))
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => {{
        let loc = std::panic::Location::caller();
        $crate::log(3, loc, format!($($arg)+))
    }};
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => {{
        let loc = std::panic::Location::caller();
        $crate::log(4, loc, format!($($arg)+))
    }};
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)+) => {{
        let loc = std::panic::Location::caller();
        $crate::log(5, loc, format!($($arg)+))
    }};
}
