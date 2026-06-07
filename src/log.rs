#[used]
#[unsafe(link_section = ".init_array")]
static INIT: fn() = || {
    let env_config = env_logger::Env::default().default_filter_or(log::Level::Info.to_string());
    let mut logger = env_logger::Builder::from_env(env_config);
    logger.init();
};

#[macro_export]
macro_rules! log_green {
    ($($args:tt)*) => {
        ::log::info!(target: module_path!(), "\x1b[32m{}\x1b[0m", format_args!($($args)*));
    };
}

#[macro_export]
macro_rules! log_red {
    ($($args:tt)*) => {
        ::log::info!(target: module_path!(), "\x1b[31m{}\x1b[0m", format_args!($($args)*));
    };
}
