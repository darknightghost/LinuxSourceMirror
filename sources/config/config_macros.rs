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
    let ret: Vec<AttributeArgument> = Vec::new();
    let splitted = split_arguments(&input_str);
    for argument_str in splitted.iter() {
        let key_value_captured = key_value_exp.captures(argument_str.as_str());
        if let Option::Some(cap) = key_value_captured {
            let key = cap.get(1).unwrap().as_str().to_string();
            let value = cap.get(2).unwrap().as_str().to_string();
            println!("key = {}, value = {}", key, value);
        } else {
            let value = argument_str;
            println!("value = {}", value);
        }
    }

    return Vec::new();
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
    if let ::syn::Data::Struct(ref mut data) = ast.data {
        if let ::syn::Fields::Named(ref _fields) = data.fields {
            for ref mut field in data.fields.iter_mut() {
                // Scan attrubutes.
                let mut new_attrs: Vec<::syn::Attribute> = Vec::new();
                for attr in field.attrs.iter() {
                    if let Option::Some(ref attr_name) = attr.path.get_ident() {
                        if attr_name.to_string() == "config_field" {
                            println!("---------------------------");
                            let args = parse_attr_args(attr.tokens.clone().into());

                            println!("{}", attr.tokens);
                            println!("---------------------------");
                        } else {
                            new_attrs.push(attr.clone());
                        }
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

    // Parse output.
    let struct_name = ast.ident.clone();
    let output = quote::quote! {
        #meta
        #ast

        impl ConfigStruct for #struct_name {
        }
    }
    .into();

    println!("{:+^40}{:+^40}", "config_struct", struct_name.to_string());
    println!("{}", output);
    println!("{:-^40}{:-^40}", "config_struct", struct_name.to_string());

    return output;
}

#[proc_macro_attribute]
pub fn config_value_map(
    _meta: ::proc_macro::TokenStream,
    _input: ::proc_macro::TokenStream,
) -> ::proc_macro::TokenStream {
    println!("\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\");
    //let ast: ::syn::DeriveInput = syn::parse(_input).unwrap();
    println!("////////////////////////////////////////");
    return _input;
}
