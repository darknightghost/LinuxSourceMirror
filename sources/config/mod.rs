//! Configs.

use crate::common;

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
        Result::Ok(value) => {
            output.load_json_value(value);
            return Result::Ok(common::Unused {});
        }
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

#[config_struct]
/// Log config.
pub struct LogConfig {
    #[config_field(key = "log_path")]
    /// Path of log file.
    log_path: ::std::path::PathBuf,

    //#[config_field(key = "log_level")]
    /// Log level.
    log_level: ::log::Level,

    #[config_field(key = "max_log_days")]
    /// Maxium days to keep logs.
    max_log_days: i32,
}

#[config_struct]
/// Service config.
pub struct ServiceConfig {
    // User.
//user: u32,

// Group.
//group: u32,
}

#[config_struct]
/// Config type.
pub struct Config {
    /// Path of config file.
    config_path: ::std::path::PathBuf,
    /*
        #[config_field(key = "/")]
        /// Service config.
        service_config: ServiceConfig,

        #[config_field(key = "/")]
        /// Log config.
        log_config: LogConfig,
    */
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
