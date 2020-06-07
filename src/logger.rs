struct Cfg {
    level: log::LevelFilter,
    bypass_stdio: bool,
}

impl Cfg {
    fn setup_logger(self) -> Result<(), fern::InitError> {
        let dispatch = fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{}[{}][{}] {}",
                    chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                    record.target(),
                    record.level(),
                    message
                ))
            })
            .level(self.level)
            .chain(fern::log_file("output.log")?);
        let dispatch = if self.bypass_stdio {
            dispatch.chain(std::io::stdout())
        } else {
            dispatch
        };
        dispatch.apply().map_err(Into::into)
    }
}

#[cfg(debug_assertions)]
pub fn setup() -> Result<(), fern::InitError> {
    Cfg {
        level: log::LevelFilter::Debug,
        bypass_stdio: false,
    }.setup_logger()
}

#[cfg(not(debug_assertions))]
pub fn setup() -> Result<(), fern::InitError> {
    Cfg {
        level: log::LevelFilter::Info,
        bypass_stdio: true,
    }.setup_logger()
}