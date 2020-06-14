use chrono::Local;
use fern::{
    colors::{Color, ColoredLevelConfig},
    log_file, Dispatch, InitError,
};
use log::LevelFilter;

struct Cfg {
    level: LevelFilter,
    bypass_stdio: bool,
}

impl Cfg {
    fn setup_logger(self) -> Result<(), InitError> {
        let file_out = Dispatch::new()
            .format(|out, message, record|
                out.finish(format_args!(
                    "{}[{}][{}] {}",
                    Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                    record.target(),
                    record.level(),
                    message,
                ))
            )
            .chain(log_file("output.log")?);
        let dispatch = Dispatch::new()
            .level(self.level)
            .chain(file_out);
        let dispatch = if self.bypass_stdio {
            dispatch
        } else {
            let colors = ColoredLevelConfig::new()
                .trace(Color::BrightBlack)
                .debug(Color::White)
                .info(Color::BrightWhite)
                .warn(Color::Yellow)
                .error(Color::Red);
            let stdout = Dispatch::new()
                .format(move |out, message, record| out.finish(format_args!(
                    "{}[{}][{}] {}",
                    Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                    record.target(),
                    colors.color(record.level()),
                    message,
                )))
            .chain(std::io::stdout());
            dispatch.chain(stdout)
        };
        dispatch.apply().map_err(Into::into)
    }
}

#[cfg(debug_assertions)]
pub fn setup() -> Result<(), InitError> {
    Cfg {
        level: LevelFilter::Debug,
        bypass_stdio: false,
    }
    .setup_logger()
}

#[cfg(not(debug_assertions))]
pub fn setup() -> Result<(), InitError> {
    Cfg {
        level: LevelFilter::Info,
        bypass_stdio: true,
    }
    .setup_logger()
}
