//! Configs.

use crate::common;
use std::io::Read;

/// Config trait.
pub trait ConfigType {
    /// Create default config object.
    ///
    fn create_default() -> Self;

    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    /// * `value`       - Json value.
    /// * `config_name` - Config name.
    /// * `full_key`    - Full key.
    fn load_json_value(
        &mut self,
        value: &::json::JsonValue,
        config_name: &String,
        full_key: &String,
    ) -> Result<common::Unused, String>;

    /// Get info string.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    fn get_info_str(&self) -> Option<String>;
}

/// Find node by path.
///
/// # Arguments
///
/// * `root`    - Root json object.
/// * `key`     - Key to load.
fn find_node_by_path<'root_lifetime>(
    root: &'root_lifetime ::json::JsonValue,
    key: &String,
) -> Result<&'root_lifetime ::json::JsonValue, String> {
    let mut ret = root;
    let mut scaned_keys: String = String::new();

    // Skip root.
    if key.len() == 0 {
        return Result::Ok(ret);
    }

    // Search value.
    for name in key.split('/') {
        if let ::json::JsonValue::Object(ref object) = ret {
            // Push key.
            if scaned_keys.len() != 0 {
                scaned_keys.push('/');
            }
            scaned_keys.push_str(name);

            // Get value.
            match object.get(name) {
                Option::Some(ref value) => {
                    ret = value;
                }

                Option::None => {
                    return Result::Err(
                        ::std::format!("Key \"{}\" not found.", scaned_keys).to_string(),
                    );
                }
            }
        } else {
            return Result::Err(
                ::std::format!("Key \"{}\" is not an object.", scaned_keys).to_string(),
            );
        }
    }

    return Result::Ok(ret);
}

/// Create config object from json config.
///
/// # Arguments
///
/// * `output`      - Output config.
/// * `root`        - Root json object.
/// * `key`         - Key to load.
/// * `optional`    - Optional.
/// * `config_name` - Config name.
/// * `full_key`    - Full key.
fn load_json_config<T: ConfigType>(
    output: &mut T,
    root: &::json::JsonValue,
    key: &String,
    optional: bool,
    config_name: &String,
    full_key: &String,
) -> Result<common::Unused, String> {
    match find_node_by_path(root, key) {
        Result::Ok(value) => match output.load_json_value(value, config_name, full_key) {
            Result::Ok(_) => {
                return Result::Ok(common::Unused {});
            }
            Result::Err(err_string) => {
                return Result::Err(err_string);
            }
        },
        Result::Err(err_string) => {
            if optional {
                return Result::Ok(common::Unused {});
            } else {
                return Result::Err(err_string);
            }
        }
    }
}

// Implements of basic types.
// Signed integer.
impl ConfigType for i64 {
    /// Create default config object.
    ///
    fn create_default() -> Self {
        return 0;
    }

    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    /// * `value`       - Json value.
    /// * `config_name` - Config name.
    /// * `full_key`    - Full key.
    fn load_json_value(
        &mut self,
        value: &::json::JsonValue,
        config_name: &String,
        full_key: &String,
    ) -> Result<common::Unused, String> {
        if let ::json::JsonValue::Number(ref json_num) = value {
            if let Option::Some(ref num) = json_num.as_fixed_point_i64(0) {
                *self = num.clone();
                return Result::Ok(common::Unused {});
            } else {
                return Result::Err(
                    ::std::format!(
                        "Failed to load config \"{}\", key = \"{}\", the value is not unsigned.",
                        config_name,
                        full_key
                    )
                    .to_string(),
                );
            }
        } else {
            return Result::Err(
                ::std::format!(
                    "Failed to load config \"{}\", key = \"{}\", the value is not a number.",
                    config_name,
                    full_key
                )
                .to_string(),
            );
        }
    }

    /// Get info string.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    fn get_info_str(&self) -> Option<String> {
        return Option::Some(::std::format!("{}", self));
    }
}

impl ConfigType for i32 {
    /// Create default config object.
    ///
    fn create_default() -> Self {
        return 0;
    }

    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    /// * `value`       - Json value.
    /// * `config_name` - Config name.
    /// * `full_key`    - Full key.
    fn load_json_value(
        &mut self,
        value: &::json::JsonValue,
        config_name: &String,
        full_key: &String,
    ) -> Result<common::Unused, String> {
        let mut val: i64 = 0;
        let ret = val.load_json_value(value, config_name, full_key);
        *self = val as i32;

        return ret;
    }

    /// Get info string.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    fn get_info_str(&self) -> Option<String> {
        return Option::Some(::std::format!("{}", self));
    }
}

impl ConfigType for i16 {
    /// Create default config object.
    ///
    fn create_default() -> Self {
        return 0;
    }

    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    /// * `value`       - Json value.
    /// * `config_name` - Config name.
    /// * `full_key`    - Full key.
    fn load_json_value(
        &mut self,
        value: &::json::JsonValue,
        config_name: &String,
        full_key: &String,
    ) -> Result<common::Unused, String> {
        let mut val: i64 = 0;
        let ret = val.load_json_value(value, config_name, full_key);
        *self = val as i16;

        return ret;
    }

    /// Get info string.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    fn get_info_str(&self) -> Option<String> {
        return Option::Some(::std::format!("{}", self));
    }
}

impl ConfigType for i8 {
    /// Create default config object.
    ///
    fn create_default() -> Self {
        return 0;
    }

    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    /// * `value`       - Json value.
    /// * `config_name` - Config name.
    /// * `full_key`    - Full key.
    fn load_json_value(
        &mut self,
        value: &::json::JsonValue,
        config_name: &String,
        full_key: &String,
    ) -> Result<common::Unused, String> {
        let mut val: i64 = 0;
        let ret = val.load_json_value(value, config_name, full_key);
        *self = val as i8;

        return ret;
    }

    /// Get info string.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    fn get_info_str(&self) -> Option<String> {
        return Option::Some(::std::format!("{}", self));
    }
}

// Unsigned integer.
impl ConfigType for u64 {
    /// Create default config object.
    ///
    fn create_default() -> Self {
        return 0;
    }

    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    /// * `value`       - Json value.
    /// * `config_name` - Config name.
    /// * `full_key`    - Full key.
    fn load_json_value(
        &mut self,
        value: &::json::JsonValue,
        config_name: &String,
        full_key: &String,
    ) -> Result<common::Unused, String> {
        if let ::json::JsonValue::Number(ref json_num) = value {
            if let Option::Some(ref num) = json_num.as_fixed_point_u64(0) {
                *self = num.clone();
                return Result::Ok(common::Unused {});
            } else {
                return Result::Err(
                    ::std::format!(
                        "Failed to load config \"{}\", key = \"{}\", the value is not unsigned.",
                        config_name,
                        full_key
                    )
                    .to_string(),
                );
            }
        } else {
            return Result::Err(
                ::std::format!(
                    "Failed to load config \"{}\", key = \"{}\", the value is not a number.",
                    config_name,
                    full_key
                )
                .to_string(),
            );
        }
    }

    /// Get info string.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    fn get_info_str(&self) -> Option<String> {
        return Option::Some(::std::format!("{}", self));
    }
}

impl ConfigType for u32 {
    /// Create default config object.
    ///
    fn create_default() -> Self {
        return 0;
    }

    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    /// * `value`       - Json value.
    /// * `config_name` - Config name.
    /// * `full_key`    - Full key.
    fn load_json_value(
        &mut self,
        value: &::json::JsonValue,
        config_name: &String,
        full_key: &String,
    ) -> Result<common::Unused, String> {
        let mut val: u64 = 0;
        let ret = val.load_json_value(value, config_name, full_key);
        *self = val as u32;

        return ret;
    }

    /// Get info string.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    fn get_info_str(&self) -> Option<String> {
        return Option::Some(::std::format!("{}", self));
    }
}

impl ConfigType for u16 {
    /// Create default config object.
    ///
    fn create_default() -> Self {
        return 0;
    }

    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    /// * `value`       - Json value.
    /// * `config_name` - Config name.
    /// * `full_key`    - Full key.
    fn load_json_value(
        &mut self,
        value: &::json::JsonValue,
        config_name: &String,
        full_key: &String,
    ) -> Result<common::Unused, String> {
        let mut val: u64 = 0;
        let ret = val.load_json_value(value, config_name, full_key);
        *self = val as u16;

        return ret;
    }

    /// Get info string.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    fn get_info_str(&self) -> Option<String> {
        return Option::Some(::std::format!("{}", self));
    }
}

impl ConfigType for u8 {
    /// Create default config object.
    ///
    fn create_default() -> Self {
        return 0;
    }

    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    /// * `value`       - Json value.
    /// * `config_name` - Config name.
    /// * `full_key`    - Full key.
    fn load_json_value(
        &mut self,
        value: &::json::JsonValue,
        config_name: &String,
        full_key: &String,
    ) -> Result<common::Unused, String> {
        let mut val: u64 = 0;
        let ret = val.load_json_value(value, config_name, full_key);
        *self = val as u8;

        return ret;
    }

    /// Get info string.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    fn get_info_str(&self) -> Option<String> {
        return Option::Some(::std::format!("{}", self));
    }
}

// boolean.
impl ConfigType for bool {
    /// Create default config object.
    ///
    fn create_default() -> Self {
        return false;
    }

    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    /// * `value`       - Json value.
    /// * `config_name` - Config name.
    /// * `full_key`    - Full key.
    fn load_json_value(
        &mut self,
        value: &::json::JsonValue,
        config_name: &String,
        full_key: &String,
    ) -> Result<common::Unused, String> {
        if let ::json::JsonValue::Boolean(ref json_bool) = value {
            *self = json_bool.clone();
            return Result::Ok(common::Unused {});
        } else {
            return Result::Err(
                ::std::format!(
                    "Failed to load config \"{}\", key = \"{}\", the value is not boolean.",
                    config_name,
                    full_key
                )
                .to_string(),
            );
        }
    }

    /// Get info string.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    fn get_info_str(&self) -> Option<String> {
        return Option::Some(::std::format!("{}", self));
    }
}

// Option
impl<T: ConfigType> ConfigType for Option<T> {
    /// Create default config object.
    ///
    fn create_default() -> Self {
        return Option::None;
    }

    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    /// * `value`       - Json value.
    /// * `config_name` - Config name.
    /// * `full_key`    - Full key.
    fn load_json_value(
        &mut self,
        value: &::json::JsonValue,
        config_name: &String,
        full_key: &String,
    ) -> Result<common::Unused, String> {
        let mut val = T::create_default();
        let ret = val.load_json_value(value, config_name, full_key);
        *self = Option::Some(val);

        return ret;
    }

    /// Get info string.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    fn get_info_str(&self) -> Option<String> {
        match self {
            Option::Some(ref val) => {
                return val.get_info_str();
            }
            Option::None => {
                return Option::Some("none".to_string());
            }
        }
    }
}

// String.
impl ConfigType for String {
    /// Create default config object.
    ///
    fn create_default() -> Self {
        return String::new();
    }

    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    /// * `value`       - Json value.
    /// * `config_name` - Config name.
    /// * `full_key`    - Full key.
    fn load_json_value(
        &mut self,
        value: &::json::JsonValue,
        config_name: &String,
        full_key: &String,
    ) -> Result<common::Unused, String> {
        *self = match value {
            ::json::JsonValue::Short(ref value_string) => value_string.to_string(),
            ::json::JsonValue::String(ref value_string) => value_string.clone(),
            _ => {
                return Result::Err(
                    ::std::format!(
                        "Failed to load config \"{}\", key = \"{}\", the value is not a string.",
                        config_name,
                        full_key
                    )
                    .to_string(),
                );
            }
        };
        return Result::Ok(common::Unused {});
    }

    /// Get info string.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    fn get_info_str(&self) -> Option<String> {
        return Option::Some(::std::format!("\"{}\"", self));
    }
}

// ::std::path::PathBuf.
impl ConfigType for ::std::path::PathBuf {
    /// Create default config object.
    ///
    fn create_default() -> Self {
        return ::std::path::PathBuf::new();
    }

    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    /// * `value`       - Json value.
    /// * `config_name` - Config name.
    /// * `full_key`    - Full key.
    fn load_json_value(
        &mut self,
        value: &::json::JsonValue,
        config_name: &String,
        full_key: &String,
    ) -> Result<common::Unused, String> {
        self.set_file_name(match value {
            ::json::JsonValue::Short(ref value_string) => value_string.to_string(),
            ::json::JsonValue::String(ref value_string) => value_string.clone(),
            _ => {
                return Result::Err(
                    ::std::format!(
                        "Failed to load config \"{}\", key = \"{}\", the value is not a string.",
                        config_name,
                        full_key
                    )
                    .to_string(),
                );
            }
        });
        return Result::Ok(common::Unused {});
    }

    /// Get info string.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    fn get_info_str(&self) -> Option<String> {
        return Option::Some(::std::format!("Path(\"{}\")", self.to_str().unwrap()));
    }
}

// ::log::Level.
impl ConfigType for ::log::Level {
    /// Create default config object.
    ///
    fn create_default() -> Self {
        return ::log::Level::Info;
    }

    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    /// * `value`       - Json value.
    /// * `config_name` - Config name.
    /// * `full_key`    - Full key.
    fn load_json_value(
        &mut self,
        value: &::json::JsonValue,
        config_name: &String,
        full_key: &String,
    ) -> Result<common::Unused, String> {
        let level_string = match value {
            ::json::JsonValue::Short(ref value_string) => value_string.to_string(),
            ::json::JsonValue::String(ref value_string) => value_string.clone(),
            _ => {
                return Result::Err(
                    ::std::format!(
                        "Failed to load config \"{}\", key = \"{}\", the value is not a string.",
                        config_name,
                        full_key
                    )
                    .to_string(),
                );
            }
        };

        *self = match level_string.as_str() {
            "Trace" => ::log::Level::Trace,
            "Debug" => ::log::Level::Debug,
            "Info" => ::log::Level::Info,
            "Warn" => ::log::Level::Warn,
            "Error" => ::log::Level::Error,
            _ => {
                return Result::Err(
                ::std::format!(
                    "Failed to load config \"{}\", key = \"{}\", illegale log level \"{}\", the log level should be \"Trace\", \"Debug\", \"Info\", \"Warn\" or \"Error\".",
                    config_name,
                    full_key,level_string
                )
                .to_string(),
            );
            }
        };

        return Result::Ok(common::Unused {});
    }

    /// Get info string.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    fn get_info_str(&self) -> Option<String> {
        let info_str = match self {
            ::log::Level::Trace => "Trace",
            ::log::Level::Debug => "Debug",
            ::log::Level::Info => "Info",
            ::log::Level::Warn => "Warn",
            ::log::Level::Error => "Error",
        };

        return Option::Some(::std::format!("{}", info_str));
    }
}

#[config_struct]
/// Log config.
pub struct LogConfig {
    #[config_field(key = "log_path")]
    /// Path of log file.
    log_path: ::std::path::PathBuf,

    #[config_field(key = "log_level")]
    /// Log level.
    log_level: ::log::Level,

    #[config_field(key = "max_log_days")]
    /// Maxium days to keep logs.
    max_log_days: i32,
}

impl LogConfig {
    /// Create new config object.
    ///
    /// # Arguments
    ///
    /// * `path` - Path of the config file
    pub fn new() -> Self {
        return LogConfig {
            log_path: ::std::path::PathBuf::new(),
            log_level: ::log::Level::Info,
            max_log_days: -1,
        };
    }
}

/// User to run as.
pub enum User {
    /// Username.
    Username(String),

    /// User ID.
    Uid(::nix::unistd::Uid),
}

// User.
impl ConfigType for User {
    /// Create default config object.
    ///
    fn create_default() -> Self {
        return User::Uid(::nix::unistd::Uid::from_raw(0));
    }

    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    /// * `value`       - Json value.
    /// * `config_name` - Config name.
    /// * `full_key`    - Full key.
    fn load_json_value(
        &mut self,
        value: &::json::JsonValue,
        config_name: &String,
        full_key: &String,
    ) -> Result<common::Unused, String> {
        *self = match value {
            ::json::JsonValue::Short(ref value_string) => User::Username(value_string.to_string()),
            ::json::JsonValue::String(ref value_string) => User::Username(value_string.to_string()),
            ::json::JsonValue::Number(ref value_num) => User::Uid(::nix::unistd::Uid::from_raw(
                value_num.as_fixed_point_u64(0).unwrap() as u32,
            )),
            _ => {
                return Result::Err("The value is not a username or UID.".to_string());
            }
        };

        return Result::Ok(common::Unused {});
    }

    /// Get info string.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    fn get_info_str(&self) -> Option<String> {
        let info_str = match self {
            User::Username(ref value) => ::std::format!("Username(\"{}\")", value),
            User::Uid(ref value) => ::std::format!("Uid({})", value),
        };

        return Option::Some(::std::format!("{}", info_str));
    }
}

/// Group to run as.
pub enum Group {
    /// Group name.
    GroupName(String),

    /// Group ID.
    Gid(::nix::unistd::Gid),
}

// Group.
impl ConfigType for Group {
    /// Create default config object.
    ///
    fn create_default() -> Self {
        return Group::Gid(::nix::unistd::Gid::from_raw(0));
    }

    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    /// * `value`       - Json value.
    /// * `config_name` - Config name.
    /// * `full_key`    - Full key.
    fn load_json_value(
        &mut self,
        value: &::json::JsonValue,
        config_name: &String,
        full_key: &String,
    ) -> Result<common::Unused, String> {
        *self = match value {
            ::json::JsonValue::Short(ref value_string) => {
                Group::GroupName(value_string.to_string())
            }
            ::json::JsonValue::String(ref value_string) => {
                Group::GroupName(value_string.to_string())
            }
            ::json::JsonValue::Number(ref value_num) => Group::Gid(::nix::unistd::Gid::from_raw(
                value_num.as_fixed_point_u64(0).unwrap() as u32,
            )),
            _ => {
                return Result::Err("The value is not a username or UID.".to_string());
            }
        };

        return Result::Ok(common::Unused {});
    }

    /// Get info string.
    ///
    /// # Arguments
    ///
    /// * `self`        - Self.
    fn get_info_str(&self) -> Option<String> {
        let info_str = match self {
            Group::GroupName(ref value) => ::std::format!("GroupName(\"{}\")", value),
            Group::Gid(ref value) => ::std::format!("Gid({})", value),
        };

        return Option::Some(::std::format!("{}", info_str));
    }
}

#[config_struct]
/// Service config.
pub struct ServiceConfig {
    #[config_field(key = "user")]
    /// User to run as.
    user: User,

    #[config_field(key = "group")]
    /// Group to run as.
    group: Group,

    #[config_field(key = "pid_file")]
    /// Path of pid file.
    pid_file: ::std::path::PathBuf,

    #[config_field(key = "data_path")]
    /// Path of data directory.
    data_path: ::std::path::PathBuf,
}

impl ServiceConfig {
    /// Create new config object.
    ///
    /// # Arguments
    ///
    /// * `path` - Path of the config file
    pub fn new() -> Self {
        return ServiceConfig {
            user: User::Uid(::nix::unistd::Uid::from_raw(0)),
            group: Group::Gid(::nix::unistd::Gid::from_raw(0)),
            pid_file: ::std::path::PathBuf::new(),
            data_path: ::std::path::PathBuf::new(),
        };
    }
}

#[config_struct]
/// Config type.
pub struct Config {
    /// Path of config file.
    config_path: ::std::path::PathBuf,

    #[config_field(key = "")]
    /// Service config.
    service_config: ServiceConfig,

    #[config_field(key = "")]
    /// Log config.
    log_config: LogConfig,
}

impl Config {
    /// Create new config object.
    ///
    pub fn new() -> Self {
        return Config {
            config_path: ::std::path::PathBuf::new(),
            service_config: ServiceConfig::new(),
            log_config: LogConfig::new(),
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
pub fn load_config(path: ::std::path::PathBuf) -> Result<common::Unused, String> {
    // Read config file.
    let mut file: ::std::fs::File = match ::std::fs::File::open(path.clone()) {
        Result::Ok(file) => file,
        Result::Err(error) => {
            return Result::Err(error.to_string());
        }
    };

    let mut config_str = String::new();

    match file.read_to_string(&mut config_str) {
        Result::Err(error) => {
            return Result::Err(error.to_string());
        }
        _ => {}
    };
    let config_str = config_str;

    // Parse json.
    let parsed_json = match ::json::parse(config_str.as_str()) {
        Result::Ok(parsed_json) => parsed_json,
        Result::Err(error) => {
            return Result::Err(::std::format!("{}", error).to_string());
        }
    };

    // Load config.
    let mut config = Config::new();
    config.config_path = path;
    if let Result::Err(error) = load_json_config(
        &mut config,
        &parsed_json,
        &("".to_string()),
        false,
        &("config".to_string()),
        &("".to_string()),
    ) {
        return Result::Err(error);
    }
    unsafe {
        CONFIG = Some(config);
    }

    return Result::Ok(common::Unused {});
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
