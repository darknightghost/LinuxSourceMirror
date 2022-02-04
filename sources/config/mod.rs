//! Configs.

/// Config trait.
trait ConfigStruct {}

#[config_struct]
/// Log config.
pub struct LogConfig {
    #[config_field(key = "/log_path", 1, b = 2)]
    /// Path of log file.
    log_path: ::std::path::PathBuf,

    #[config_field(key = "/log_level")]
    /// Log level.
    log_level: ::log::Level,

    //#[config_field(key = "/max_log_days")]
    #[config_field]
    /// Maxium days to keep logs.
    max_log_days: i32,
}

/// Service config.
pub struct ServiceConfig {
    /// User.
    user: u32,

    /// Group.
    group: u32,
}

/// Config type.
pub struct Config {
    /// Path of config file.
    config_path: ::std::path::PathBuf,
}

impl Config {
    /// Create new config object.
    ///
    /// # Arguments
    ///
    /// * `path` - Path of the config file
    pub fn new(path: ::std::path::PathBuf) -> Self {
        return Config { config_path: path };
    }
}

/// Config.
static mut CONFIG: Option<Config> = Option::None;

/// Load config, only call this method before the service initialized.
///
/// # Arguments
///
/// * `path` - Path of the config file
pub fn load_config(path: ::std::path::PathBuf) {
    unsafe {
        CONFIG = Some(Config::new(path));
    }
}

/// Get config.
///
/// # Panics
///
/// A panic will occured when config::load_config() has not been called
/// before this method called.
pub fn get_config() -> &'static Config {
    unsafe {
        return CONFIG.as_ref().unwrap();
    }
}
