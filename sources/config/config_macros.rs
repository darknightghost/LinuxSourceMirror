//! Macros of configs.

extern crate lazy_static;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate regex;
extern crate syn;

/// Argument of attribute.
struct AttributeArgument {
    /// Key.
    key: Option<String>,

    // Value.
    value: Box<dyn::std::any::Any>,
}

impl ::std::fmt::Display for AttributeArgument {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        // Key.
        let mut key_str = "None".to_string();
        if let Option::Some(ref key) = self.key {
            key_str = ::std::format!("\"{}\"", key);
        }

        // Value.
        let mut value_str = "Unknow Type".to_string();
        if self.value.is::<String>() {
            let val_ref: &String = self.value.downcast_ref::<String>().unwrap();
            value_str = ::std::format!("\"{}\"", val_ref);
        } else if self.value.is::<i64>() {
            let val_ref: &i64 = self.value.downcast_ref::<i64>().unwrap();
            value_str = ::std::format!("{}", val_ref);
        }

        return write!(formatter, "key: {}, value: {}", key_str, value_str);
    }
}

/// Parse value type.
///
/// # Arguments
///
/// * `value_str` - Value string.
fn parse_value_str(value_str: String) -> Box<dyn::std::any::Any> {
    // Is integer?
    let exp = ::regex::Regex::new("^([+-]?\\d+)$").unwrap();
    let cap_result = exp.captures(value_str.as_str());
    if let Option::Some(cap) = cap_result {
        let ret: i64 = cap
            .get(1)
            .unwrap()
            .as_str()
            .to_string()
            .parse::<i64>()
            .unwrap();
        return Box::new(ret);
    }

    // Is string?
    let exp = ::regex::Regex::new("^\"(.*)\"$").unwrap();
    let cap_result = exp.captures(value_str.as_str());
    if let Option::Some(cap) = cap_result {
        let ret: String = cap.get(1).unwrap().as_str().to_string();
        return Box::new(ret);
    }

    panic!("Unsupported format!");
}

/// Split arguments.
///
/// # Arguments
///
/// * `input` - Input string.
fn split_arguments(input: &String) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    let mut iter = input.chars();
    let mut buffer: String = String::new();
    loop {
        let current = iter.next();
        if let Option::Some(ch) = current {
            if ch.is_whitespace() && buffer.len() == 0 {
                continue;
            } else if ch == ',' {
                ret.push(buffer);
                buffer = String::new();
                buffer.clear();
            } else if ch == '(' || ch == '[' || ch == '{' {
                // Get end bracket.
                let mut end_bracket = ')';
                if ch == '[' {
                    end_bracket = ']'
                } else if ch == '{' {
                    end_bracket = '}'
                }
                let end_bracket = end_bracket;

                // Push character.
                buffer.push(ch);

                // Search end character.
                loop {
                    let current = iter.next();
                    if let Option::Some(ch) = current {
                        if ch == end_bracket {
                            buffer.push(ch);
                            break;
                        } else {
                            buffer.push(ch);
                        }
                    } else {
                        panic!("Illegal argument \"({})\".", input);
                    }
                }
            } else {
                // Push character.
                buffer.push(ch);
            }
        } else {
            if buffer.len() > 0 {
                ret.push(buffer);
                break;
            }
        }
    }

    return ret;
}

/// Parse arguments of attributs, only support string now.
///
/// # Arguments
///
/// * `input` - Input token stream.
fn parse_attr_args(input: ::proc_macro::TokenStream) -> Vec<AttributeArgument> {
    let mut input_str = input.to_string();

    if input_str == "" {
        return Vec::new();
    }

    // Check input.
    let argument_exp = ::regex::Regex::new("^\\((.*)\\)$").unwrap();
    let args_captured = argument_exp.captures(input_str.as_str());
    if let Option::Some(cap) = args_captured {
        input_str = cap.get(1).unwrap().as_str().to_string();
    } else {
        panic!("Illegal argument \"{}\".", input_str);
    }

    // Parse.
    let key_value_exp = ::regex::Regex::new("^([_a-zA-Z][_0-9a-zA-Z]*)\\s*=\\s*(.*)$").unwrap();
    let mut ret: Vec<AttributeArgument> = Vec::new();
    let splitted = split_arguments(&input_str);
    for argument_str in splitted.iter() {
        let key_value_captured = key_value_exp.captures(argument_str.as_str());
        if let Option::Some(cap) = key_value_captured {
            let key = cap.get(1).unwrap().as_str().to_string();
            let value = cap.get(2).unwrap().as_str().to_string();
            ret.push(AttributeArgument {
                key: Option::Some(key),
                value: parse_value_str(value),
            });
        } else {
            let value = argument_str.to_string();
            ret.push(AttributeArgument {
                key: Option::None,
                value: parse_value_str(value),
            });
        }
    }

    return ret;
}

/// Config field infomation.
struct ConfigFieldInfo {
    /// Field name.
    field_name: ::proc_macro2::Ident,

    /// Key.
    key: String,
}

impl ConfigFieldInfo {
    fn new(args: Vec<AttributeArgument>, field_name: proc_macro2::Ident) -> ConfigFieldInfo {
        let mut key: Option<String> = Option::None;

        // Parse arguments.
        for arg in args.iter() {
            match arg.key {
                Option::Some(ref key_name) => match key_name.as_str() {
                    "key" => {
                        key = Option::Some(
                            arg.value
                                .as_ref()
                                .downcast_ref::<String>()
                                .unwrap()
                                .to_string(),
                        );
                    }
                    _ => {
                        panic!(
                            "Illegal keyword argument \"{}\" of attribute \"config_field\"!",
                            key_name
                        );
                    }
                },
                _ => {
                    panic!("Illegal position argument of attribute \"config_field\"!");
                }
            }
        }

        // Check.
        if let Option::None = key {
            panic!("Missing keyword argument \"key\" of attribute \"config_field\"!");
        }

        return ConfigFieldInfo {
            field_name: field_name,
            key: key.unwrap(),
        };
    }
}

#[proc_macro_attribute]
pub fn config_struct(
    _meta: ::proc_macro::TokenStream,
    _input: ::proc_macro::TokenStream,
) -> ::proc_macro::TokenStream {
    // Convert meta.
    let meta = ::proc_macro2::TokenStream::from(_meta);

    // Parse input data to ast.
    let mut ast: ::syn::DeriveInput = ::syn::parse(_input).unwrap();

    // Scan fields.
    let mut fields_info: Vec<ConfigFieldInfo> = Vec::new();
    if let ::syn::Data::Struct(ref mut data) = ast.data {
        if let ::syn::Fields::Named(ref _fields) = data.fields {
            for ref mut field in data.fields.iter_mut() {
                // Scan attrubutes.
                let mut new_attrs: Vec<::syn::Attribute> = Vec::new();
                for attr in field.attrs.iter() {
                    if let Option::Some(ref attr_name) = attr.path.get_ident() {
                        if attr_name.to_string() == "config_field" {
                            // Parse attributes.
                            let args = parse_attr_args(attr.tokens.clone().into());
                            if let Option::Some(ref ident) = field.ident {
                                let field_name = ident.clone();
                                fields_info.push(ConfigFieldInfo::new(args, field_name));
                            } else {
                                panic!("Only structs with named fields can have attribute `config_struct`.");
                            }
                        } else {
                            new_attrs.push(attr.clone());
                        }
                    } else {
                        panic!("Missing argument(s) of attribute \"config_field\"!");
                    }
                }
                field.attrs = new_attrs;
            }
        } else {
            panic!("Only structs with named fields can have attribute `config_struct`.");
        }
    } else {
        panic!("Only structs with named fields can have attribute `config_struct`.");
    }

    // Generate load code.
    let mut load_code = ::quote::quote! {};
    for info in fields_info.iter() {
        let field = info.field_name.clone();
        let key = info.key.clone();

        load_code = ::quote::quote! {
            #load_code

            ret = load_json_config(&mut self.#field, value, &#key.to_string());
            if let Result::Err(ref s) = ret {
                return ret;
            }
        };
    }

    // Parse output.
    let struct_name = ast.ident.clone();
    let output = quote::quote! {
        #meta
        #ast


        impl ConfigType for #struct_name {
            /// Load json value.
            ///
            /// # Arguments
            ///
            /// * `self`    - Self.
            /// * `value`   - Json value.
            fn load_json_value(
                &mut self,
                value: &::json::JsonValue) -> Result<common::Unused, String> {
                let mut ret = Result::Ok(common::Unused{});

                #load_code

                return ret;
            }
        }
    }
    .into();

    println!("{:+^40}{:+^40}", "config_struct", struct_name.to_string());
    println!("{}", output);
    println!("{:-^40}{:-^40}", "config_struct", struct_name.to_string());

    return output;
}
