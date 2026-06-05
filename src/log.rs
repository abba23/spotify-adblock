#[used]
#[unsafe(link_section = ".init_array")]
static INIT: unsafe extern "C" fn() = init;

unsafe extern "C" fn init() {
    let env_config = env_logger::Env::default().default_filter_or("info");
    let mut logger = env_logger::Builder::from_env(env_config);
    if let Err(error) = logger.try_init() {
        panic!("Failed to initialize logger: {error}.");
    }
}

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
