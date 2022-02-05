//! Configs.

use crate::common;
use std::io::Read;

/// Config trait.
pub trait ConfigType {
    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`    - Self.
    /// * `value`   - Json value.
    fn load_json_value(&mut self, value: &::json::JsonValue) -> Result<common::Unused, String>;
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
    //TODO
    println!("-----------------{}------------", key);
    for name in key.split('/') {
        println!("{}", name);
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
/// * `root`    - Root json object.
/// * `key`     - Key to load.
fn load_json_config<T: ConfigType>(
    output: &mut T,
    root: &::json::JsonValue,
    key: &String,
) -> Result<common::Unused, String> {
    match find_node_by_path(root, key) {
        Result::Ok(value) => match output.load_json_value(value) {
            Result::Ok(_) => {
                return Result::Ok(common::Unused {});
            }
            Result::Err(err_string) => {
                return Result::Err(err_string);
            }
        },
        Result::Err(err_string) => {
            return Result::Err(err_string);
        }
    }
}

// Implements of basic types.
// Signed integer.
impl ConfigType for i64 {
    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`    - Self.
    /// * `value`   - Json value.
    fn load_json_value(&mut self, value: &::json::JsonValue) -> Result<common::Unused, String> {
        if let ::json::JsonValue::Number(ref json_num) = value {
            if let Option::Some(ref num) = json_num.as_fixed_point_i64(0) {
                *self = num.clone();
                return Result::Ok(common::Unused {});
            } else {
                return Result::Err("The value is not unsigned.".to_string());
            }
        } else {
            return Result::Err("The value is not a number.".to_string());
        }
    }
}

impl ConfigType for i32 {
    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`    - Self.
    /// * `value`   - Json value.
    fn load_json_value(&mut self, value: &::json::JsonValue) -> Result<common::Unused, String> {
        let mut val: i64 = 0;
        let ret = val.load_json_value(value);
        *self = val as i32;

        return ret;
    }
}

impl ConfigType for i16 {
    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`    - Self.
    /// * `value`   - Json value.
    fn load_json_value(&mut self, value: &::json::JsonValue) -> Result<common::Unused, String> {
        let mut val: i64 = 0;
        let ret = val.load_json_value(value);
        *self = val as i16;

        return ret;
    }
}

impl ConfigType for i8 {
    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`    - Self.
    /// * `value`   - Json value.
    fn load_json_value(&mut self, value: &::json::JsonValue) -> Result<common::Unused, String> {
        let mut val: i64 = 0;
        let ret = val.load_json_value(value);
        *self = val as i8;

        return ret;
    }
}

// Unsigned integer.
impl ConfigType for u64 {
    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`    - Self.
    /// * `value`   - Json value.
    fn load_json_value(&mut self, value: &::json::JsonValue) -> Result<common::Unused, String> {
        if let ::json::JsonValue::Number(ref json_num) = value {
            if let Option::Some(ref num) = json_num.as_fixed_point_u64(0) {
                *self = num.clone();
                return Result::Ok(common::Unused {});
            } else {
                return Result::Err("The value is not unsigned.".to_string());
            }
        } else {
            return Result::Err("The value is not a number.".to_string());
        }
    }
}

impl ConfigType for u32 {
    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`    - Self.
    /// * `value`   - Json value.
    fn load_json_value(&mut self, value: &::json::JsonValue) -> Result<common::Unused, String> {
        let mut val: u64 = 0;
        let ret = val.load_json_value(value);
        *self = val as u32;

        return ret;
    }
}

impl ConfigType for u16 {
    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`    - Self.
    /// * `value`   - Json value.
    fn load_json_value(&mut self, value: &::json::JsonValue) -> Result<common::Unused, String> {
        let mut val: u64 = 0;
        let ret = val.load_json_value(value);
        *self = val as u16;

        return ret;
    }
}

impl ConfigType for u8 {
    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`    - Self.
    /// * `value`   - Json value.
    fn load_json_value(&mut self, value: &::json::JsonValue) -> Result<common::Unused, String> {
        let mut val: u64 = 0;
        let ret = val.load_json_value(value);
        *self = val as u8;

        return ret;
    }
}

// String.
impl ConfigType for String {
    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`    - Self.
    /// * `value`   - Json value.
    fn load_json_value(&mut self, value: &::json::JsonValue) -> Result<common::Unused, String> {
        if let ::json::JsonValue::String(ref value_string) = value {
            *self = value_string.clone();
            return Result::Ok(common::Unused {});
        } else {
            return Result::Err("The value is not a string.".to_string());
        }
    }
}

// ::std::path::PathBuf.
impl ConfigType for ::std::path::PathBuf {
    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`    - Self.
    /// * `value`   - Json value.
    fn load_json_value(&mut self, value: &::json::JsonValue) -> Result<common::Unused, String> {
        if let ::json::JsonValue::String(ref value_string) = value {
            self.set_file_name(value_string);
            return Result::Ok(common::Unused {});
        } else {
            return Result::Err("The value is not a string.".to_string());
        }
    }
}

// ::log::Level.
impl ConfigType for ::log::Level {
    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`    - Self.
    /// * `value`   - Json value.
    fn load_json_value(&mut self, value: &::json::JsonValue) -> Result<common::Unused, String> {
        if let ::json::JsonValue::String(ref value_string) = value {
            *self = match value_string.as_str() {
                "Trace" => ::log::Level::Trace,
                "Debug" => ::log::Level::Debug,
                "Info" => ::log::Level::Info,
                "Warn" => ::log::Level::Warn,
                "Error" => ::log::Level::Error,
                _ => {
                    return Result::Err(
                        ::std::format!("Illegale log level \"{}\".", value_string).to_string(),
                    );
                }
            };
            return Result::Ok(common::Unused {});
        } else {
            return Result::Err("The value is not a string.".to_string());
        }
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
    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`    - Self.
    /// * `value`   - Json value.
    fn load_json_value(&mut self, value: &::json::JsonValue) -> Result<common::Unused, String> {
        *self = match value {
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
    /// Load json value.
    ///
    /// # Arguments
    ///
    /// * `self`    - Self.
    /// * `value`   - Json value.
    fn load_json_value(&mut self, value: &::json::JsonValue) -> Result<common::Unused, String> {
        *self = match value {
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
    /// # Arguments
    ///
    /// * `path` - Path of the config file
    pub fn new(path: ::std::path::PathBuf) -> Self {
        return Config {
            config_path: path,
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
    let mut config = Config::new(path);
    if let Result::Err(error) = load_json_config(&mut config, &parsed_json, &("".to_string())) {
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
