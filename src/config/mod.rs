//! Configs.

/// Config type.
pub struct Config {
    /// Path of config file.
    m_config_path: ::std::path::PathBuf,
}

impl Config {
    /// Create new config object.
    ///
    /// # Arguments
    ///
    /// * `path` - Path of the config file
    pub fn new(path: ::std::path::PathBuf) -> Self {
        return Config {
            m_config_path: path,
        };
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
pub fn config() -> &'static Config {
    unsafe {
        return CONFIG.as_ref().unwrap();
    }
}
