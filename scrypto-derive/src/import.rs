use std::fs;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::*;

macro_rules! trace {
    ($($arg:expr),*) => {{
        #[cfg(feature = "trace")]
        println!($($arg),*);
    }};
}

pub fn handle_import(input: TokenStream) -> TokenStream {
    trace!("handle_import() starts");
    let span = Span::call_site();

    let path_lit: LitStr = parse2(input).expect("Unable to parse input");
    let path = path_lit.value();
    let abi = fs::read_to_string(path).expect("Unable to load Abi");
    let component: scrypto_abi::Component =
        serde_json::from_str(abi.as_str()).expect("Unable to parse Abi");
    trace!("ABI: {:?}", component);

    let mut items: Vec<Item> = vec![];
    let mut implementations: Vec<ItemImpl> = vec![];

    let ident = Ident::new(component.name.as_str(), span);
    trace!("Component name: {}", quote! { #ident });

    let structure: Item = parse_quote! {
        pub struct #ident {
            address: scrypto::types::Address
        }
    };
    items.push(structure);

    let mut functions = Vec::<ItemFn>::new();
    functions.push(parse_quote! {
        pub fn from_address(address: scrypto::types::Address) -> Self {
            Self {
                address
            }
        }
    });

    for method in &component.methods {
        trace!("Processing method: {:?}", method);

        let func_indent = Ident::new(method.name.as_str(), span);
        let mut func_inputs = Punctuated::<FnArg, Comma>::new();

        match method.mutability {
            scrypto_abi::Mutability::Immutable => func_inputs.push(parse_quote! { &self }),
            scrypto_abi::Mutability::Mutable => func_inputs.push(parse_quote! { &mut self }),
            _ => {}
        }

        for (i, input) in method.inputs.iter().enumerate() {
            match input {
                _ => {
                    let ident = format_ident!("arg{}", i);
                    let (new_type, new_items) = get_native_type(input);
                    func_inputs.push(parse_quote! { #ident: #new_type });
                    items.extend(new_items);
                }
            }
            if i < method.inputs.len() - 1 {
                func_inputs.push_punct(Comma(span));
            }
        }
        let (func_output, new_items) = get_native_type(&method.output);
        items.extend(new_items);

        functions.push(parse_quote! {
            pub fn #func_indent(#func_inputs) -> #func_output {
                todo!()
            }
        });
    }

    let implementation = parse_quote! {
        impl #ident {
            #(#functions)*
        }
    };
    trace!("Implementation: {}", quote! { #implementation });
    implementations.push(implementation);

    let output = quote! {
         #(#items)*

         #(#implementations)*
    };
    trace!("handle_import() finishes");

    #[cfg(feature = "trace")]
    crate::utils::print_compiled_code("import!", &output);

    output.into()
}

fn get_native_type(ty: &sbor::Type) -> (Type, Vec<Item>) {
    let mut items = Vec::<Item>::new();

    let t: Type = match ty {
        sbor::Type::Unit => parse_quote! { () },
        sbor::Type::Bool => parse_quote! { bool },
        sbor::Type::I8 => parse_quote! { i8 },
        sbor::Type::I16 => parse_quote! { i16 },
        sbor::Type::I32 => parse_quote! { i32 },
        sbor::Type::I64 => parse_quote! { i64 },
        sbor::Type::I128 => parse_quote! { i128 },
        sbor::Type::U8 => parse_quote! { u8 },
        sbor::Type::U16 => parse_quote! { u16 },
        sbor::Type::U32 => parse_quote! { u32 },
        sbor::Type::U64 => parse_quote! { u64 },
        sbor::Type::U128 => parse_quote! { u128 },
        sbor::Type::String => parse_quote! { String },
        sbor::Type::Option { value } => {
            let (new_type, new_items) = get_native_type(value);
            items.extend(new_items);

            parse_quote! { Option<#new_type> }
        }
        sbor::Type::Box { value } => {
            let (new_type, new_items) = get_native_type(value);
            items.extend(new_items);

            parse_quote! { Box<#new_type> }
        }
        sbor::Type::Struct { name, fields } => {
            let ident = format_ident!("{}", name);

            match fields {
                sbor::FieldTypes::Named { fields } => {
                    let names: Vec<Ident> = fields.keys().map(|k| format_ident!("{}", k)).collect();
                    let mut types: Vec<Type> = vec![];
                    for v in fields.values() {
                        let (new_type, new_items) = get_native_type(v);
                        types.push(new_type);
                        items.extend(new_items);
                    }
                    items.push(parse_quote! {
                        #[derive(Debug, serde::Serialize, serde::Deserialize)]
                        pub struct #ident {
                            #( pub #names : #types, )*
                        }
                    });
                }
                _ => {
                    todo!("Add support for non-named fields")
                }
            }

            parse_quote! { #ident }
        }
        sbor::Type::Tuple { elements } => {
            let mut types: Vec<Type> = vec![];

            for element in elements {
                let (new_type, new_items) = get_native_type(element);
                types.push(new_type);
                items.extend(new_items);
            }

            parse_quote! { ( #(#types),* ) }
        }
        sbor::Type::Array { element, length } => {
            let (new_type, new_items) = get_native_type(element);
            items.extend(new_items);

            let n = *length as usize;
            parse_quote! { [#new_type; #n] }
        }
        sbor::Type::Enum { name, variants } => {
            let ident = format_ident!("{}", name);
            let mut native_variants = Vec::<Variant>::new();

            for (v_name, v_fields) in variants {
                let v_ident = format_ident!("{}", v_name);

                match v_fields {
                    sbor::FieldTypes::Named { fields } => {
                        let mut names: Vec<Ident> = vec![];
                        let mut types: Vec<Type> = vec![];
                        for (n, v) in fields {
                            names.push(format_ident!("{}", n));
                            let (new_type, new_items) = get_native_type(v);
                            types.push(new_type);
                            items.extend(new_items);
                        }
                        native_variants.push(parse_quote! {
                            #v_ident {
                                #(#names: #types),*
                            }
                        });
                    }
                    sbor::FieldTypes::Unnamed { fields } => {
                        let mut types: Vec<Type> = vec![];
                        for v in fields {
                            let (new_type, new_items) = get_native_type(v);
                            types.push(new_type);
                            items.extend(new_items);
                        }
                        native_variants.push(parse_quote! {
                            #v_ident ( #(#types),* )
                        });
                    }
                    sbor::FieldTypes::Unit => {
                        native_variants.push(parse_quote! {
                            #v_ident
                        });
                    }
                };
            }

            items.push(parse_quote! {
                #[derive(Debug, serde::Serialize, serde::Deserialize)]
                pub enum #ident {
                    #( #native_variants ),*
                }
            });

            parse_quote! { #ident }
        }
        sbor::Type::Vec { element } => {
            let (new_type, new_items) = get_native_type(element);
            items.extend(new_items);

            parse_quote! { Vec<#new_type> }
        }
        sbor::Type::TreeSet { element } => {
            let (new_type, new_items) = get_native_type(element);
            items.extend(new_items);

            parse_quote! { BTreeSet<#new_type> }
        }
        sbor::Type::TreeMap { key, value } => {
            let (key_type, new_items) = get_native_type(key);
            items.extend(new_items);
            let (value_type, new_items) = get_native_type(value);
            items.extend(new_items);

            parse_quote! { BTreeMap<#key_type, #value_type> }
        }
        sbor::Type::HashSet { element } => {
            let (new_type, new_items) = get_native_type(element);
            items.extend(new_items);

            parse_quote! { HashSet<#new_type> }
        }
        sbor::Type::HashMap { key, value } => {
            let (key_type, new_items) = get_native_type(key);
            items.extend(new_items);
            let (value_type, new_items) = get_native_type(value);
            items.extend(new_items);

            parse_quote! { HashMap<#key_type, #value_type> }
        }
    };

    (t, items)
}
