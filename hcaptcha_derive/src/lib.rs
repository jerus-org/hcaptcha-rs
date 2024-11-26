#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(rustdoc_missing_doc_code_examples))]
#![cfg_attr(docsrs, warn(rustdoc::invalid_codeblock_attributes))]

//! Derive macro for hcaptcha::Hcaptcha
//!
//! Derives the Hcaptcha trait requirements for a struct such as in the example
//! below of a contact form providing the hcaptcha response token, the
//! client ip address of the client and the Hcaptcha service sitekey.
//!
//! ```rust
//! use hcaptcha::Hcaptcha;
//!
//! #[derive(Hcaptcha)]
//! pub struct ContactForm {
//!     name: String,
//!     phone: String,
//!     email: String,
//!     message: String,
//!     #[captcha]
//!     hcaptcha: String,
//!     #[remoteip]
//!     ip: String,
//!     #[sitekey]
//!     key: String,
//! }
//!```
//!
//! The derive macro provides code such as the following:
//!
//!```rust
//! # use hcaptcha::Hcaptcha;
//! #
//! # pub struct ContactForm {
//! #     name: String,
//! #     phone: String,
//! #     email: String,
//! #     message: String,
//! #     hcaptcha: String,
//! #     ip: String,
//! #     key: String,
//! # }
//! impl Hcaptcha for ContactForm {
//!     fn valid_response(
//!         &self,
//!         secret: &str,
//!         uri: Option<String>,
//!     ) -> std::pin::Pin<
//!         Box<
//!             dyn std::future::Future<
//!                     Output = Result<hcaptcha::Response,
//!                                     hcaptcha::Error>,
//!                                     >,
//!         >,
//!     > {
//!         let mut client = hcaptcha::Client::new();
//!         if let Some(u) = uri {
//!             match client.set_url(&u) {
//!                 Ok(c) => client = c,
//!                 Err(e) => {
//!                     return Box::pin(async { Err(e) });
//!                 }
//!             };
//!         };
//!         #[allow(unused_mut)]
//!         let mut captcha;
//!         match hcaptcha::Captcha::new(&self.hcaptcha) {
//!             Ok(c) => captcha = c,
//!             Err(e) => {
//!                 return Box::pin(async { Err(e) });
//!             }
//!         };
//!         match captcha.set_remoteip(&self.ip) {
//!             Ok(c) => captcha = c,
//!             Err(e) => {
//!                 return Box::pin(async { Err(e) });
//!             }
//!         };
//!         match captcha.set_sitekey(&self.key) {
//!             Ok(c) => captcha = c,
//!             Err(e) => {
//!                 return Box::pin(async { Err(e) });
//!             }
//!         };
//!         let request;
//!         match hcaptcha::Request::new(&secret, captcha) {
//!             Ok(r) => request = r,
//!             Err(e) => {
//!                 return Box::pin(async { Err(e) });
//!             }
//!         };
//!         Box::pin(client.verify(request))
//!     }
//! }
//!```

extern crate proc_macro;

use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro_error2::proc_macro_error;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput};

/// Derive the Hcaptcha trait for a struct.
///
/// # Example
///
/// ```rust
/// use hcaptcha::Hcaptcha;
///
/// #[derive(Hcaptcha)]
/// pub struct ContactForm {
///     name: String,
///     email: String,
///     phone: String,
///     message: String,
///     #[captcha]
///     token: String,
/// }
/// ```
#[proc_macro_error]
#[proc_macro_derive(Hcaptcha, attributes(captcha, remoteip, sitekey))]
pub fn hcaptcha_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hcaptcha(&ast)
}

fn impl_hcaptcha(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let data = &ast.data;

    let data_struct = get_struct_data(data, name);

    let attributes = get_attributes(data_struct);

    let captcha = get_required_attribute(&attributes, "captcha", name);

    let remoteip = get_optional_attribute(&attributes, "remoteip", "set_remoteip");
    let sitekey = get_optional_attribute(&attributes, "sitekey", "set_sitekey");

    let gen = quote! {
        impl #impl_generics Hcaptcha for #name #ty_generics #where_clause {
            fn valid_response(&self, secret: &str, uri: Option<String>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<hcaptcha::Response, hcaptcha::Error>> >>  {
                let mut client = hcaptcha::Client::new();
                if let Some(u) = uri {
                        match client.set_url(&u)
                         {
                            Ok(c) => client = c,
                            Err(e) => {
                                return Box::pin(async { Err(e) });
                            }
                        };
                };

                #captcha
                #remoteip
                #sitekey;
                let request;
                match hcaptcha::Request::new(&secret, captcha) {
                    Ok(r) => request = r,
                    Err(e) => {
                        return Box::pin(async { Err(e) } );
                    }
                };
                Box::pin(client.verify(request))
            }
        }
    };
    gen.into()
}

/// Generate tokens for optional attribute
///
/// # inputs
/// - attributes:   Hashmap of attributes
/// - name:         Name of the attribute to process
/// - method:       Name of the method to insert
///
/// # Output
///
/// Token stream for the method if the named attributed is found
/// Empty token stream of the attribute is not found
///
fn get_optional_attribute(
    attributes: &HashMap<String, &proc_macro2::Ident>,
    name: &str,
    method: &str,
) -> proc_macro2::TokenStream {
    let method = quote::format_ident!("{}", method);
    match attributes.get(name) {
        Some(i) => {
            let i = <&proc_macro2::Ident>::clone(i);
            quote! {
                match captcha.#method(&self.#i) {
                    Ok(c) => captcha = c,
                    Err(e) => {
                        return Box::pin(async { Err(e) });
                    }
                };
            }
        }
        None => quote! {},
    }
}

/// Generate tokens for required attribute
///
/// # inputs
/// - attributes:   Hashmap of attributes
/// - name:         Name of the attribute to process
///
/// # Output
///
/// Token stream for the method if the named attributed is found
/// Generate compiler error if not found
///
fn get_required_attribute(
    attributes: &HashMap<String, &proc_macro2::Ident>,
    name: &str,
    id: &proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    match attributes.get(name) {
        Some(i) => {
            let i = <&proc_macro2::Ident>::clone(i);
            quote! {
                #[allow(unused_mut)]
                let mut captcha;
                match hcaptcha::Captcha::new(&self.#i) {
                    Ok(c) => captcha = c,
                    Err(e) => {
                        return Box::pin(async{Err(e)});
                    }
                };
            }
        }
        None => {
            let example = r#"
        #[derive(Hcaptcha)]
        struct MyStruct {
            #[captcha]
            hcaptcha: String,
        }"#;
            proc_macro_error2::abort! {id,
                "Field containing hcaptcha not identified";
                help = "The field containing the hcaptcha response string must be identified with #[captcha]
                {}", &example;
            };
        }
    }
}

/// Extract the tokens for the Struct from the data
/// If the data is not stored in the Struct variant abort compilation
/// with an error.
fn get_struct_data<'a>(data: &'a Data, name: &Ident) -> &'a DataStruct {
    match data {
        Data::Struct(s) => s,
        _ => {
            let example = r#"
        #[derive(Hcaptcha)]
        struct MyStruct {
            #[captcha]
            hcaptcha: String,
        }"#;
            proc_macro_error2::abort! {name,
                "Must derive on a struct";
                help = "This macro can only be implemented on a struct.
                {}", &example;
            };
        }
    }
}

/// Iterate through the fields in the struct to find the attributes
/// that identify the fields relevant to hcaptcha processing
fn get_attributes(data_struct: &DataStruct) -> HashMap<String, &Ident> {
    let mut attributes = HashMap::new();

    let _: Vec<String> = data_struct
        .fields
        .iter()
        .filter(|f| !f.attrs.is_empty())
        .map(|f| {
            let a = f.attrs[0].path().get_ident().unwrap().to_string();
            if let Some(i) = &f.ident {
                attributes.insert(a.clone(), i);
            }
            a
        })
        .collect();

    attributes
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use quote::format_ident;
    use std::iter::FromIterator;
    use syn::parse::Parser;
    use syn::{Attribute, Field, Fields, FieldsNamed};

    use super::*;

    #[test]
    fn test_get_optional_attribute_with_valid_attribute() {
        let mut attributes = HashMap::new();
        let ident = Ident::new("test_field", Span::call_site());
        attributes.insert("test".to_string(), &ident);

        let result = get_optional_attribute(&attributes, "test", "test_method");

        let expected = quote! {
            match captcha.test_method(&self.test_field) {
                Ok(c) => captcha = c,
                Err(e) => {
                    return Box::pin(async { Err(e) });
                }
            };
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_get_optional_attribute_with_missing_attribute() {
        let attributes = HashMap::new();

        let result = get_optional_attribute(&attributes, "test", "test_method");

        assert_eq!(result.to_string(), quote! {}.to_string());
    }

    #[test]
    fn test_get_optional_attribute_with_different_method_name() {
        let mut attributes = HashMap::new();
        let ident = Ident::new("field", Span::call_site());
        attributes.insert("attr".to_string(), &ident);

        let result = get_optional_attribute(&attributes, "attr", "custom_method");

        let expected = quote! {
            match captcha.custom_method(&self.field) {
                Ok(c) => captcha = c,
                Err(e) => {
                    return Box::pin(async { Err(e) });
                }
            };
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn test_get_required_attribute_with_valid_field() {
        let mut attributes = HashMap::new();
        let field_ident = Ident::new("hcaptcha_field", Span::call_site());
        let struct_ident = Ident::new("TestStruct", Span::call_site());

        attributes.insert("captcha".to_string(), &field_ident);

        let result = get_required_attribute(&attributes, "captcha", &struct_ident);
        assert!(!result.is_empty());
    }

    // #[test]
    // #[should_panic]
    // fn test_get_required_attribute_missing_field() {
    //     let attributes = HashMap::new();
    //     let struct_ident = Ident::new("TestStruct", Span::call_site());

    //     get_required_attribute(&attributes, "captcha", &struct_ident);
    // }

    #[test]
    fn test_get_required_attribute_wrong_field_name() {
        let mut attributes = HashMap::new();
        let field_ident = Ident::new("hcaptcha_field", Span::call_site());
        let struct_ident = Ident::new("TestStruct", Span::call_site());

        attributes.insert("wrong_name".to_string(), &field_ident);

        std::panic::catch_unwind(|| {
            get_required_attribute(&attributes, "captcha", &struct_ident);
        })
        .expect_err("Should panic when captcha field is not found");
    }

    #[test]
    fn test_get_required_attribute_multiple_fields() {
        let mut attributes = HashMap::new();
        let captcha_field = Ident::new("hcaptcha_field", Span::call_site());
        let other_field = Ident::new("other_field", Span::call_site());
        let struct_ident = Ident::new("TestStruct", Span::call_site());

        attributes.insert("captcha".to_string(), &captcha_field);
        attributes.insert("other".to_string(), &other_field);

        let result = get_required_attribute(&attributes, "captcha", &struct_ident);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_get_struct_data_valid_struct() {
        let name = Ident::new("TestStruct", Span::call_site());
        let fields = Fields::Named(FieldsNamed {
            brace_token: Default::default(),
            named: Default::default(),
        });
        let data_struct = DataStruct {
            struct_token: Default::default(),
            fields,
            semi_token: None,
        };
        let data = Data::Struct(data_struct.clone());

        let result = get_struct_data(&data, &name);
        assert_eq!(result, &data_struct);
    }

    // #[test]
    // #[should_panic(expected = "Must derive on a struct")]
    // fn test_get_struct_data_enum() {
    //     let name = Ident::new("TestEnum", Span::call_site());
    //     let data = Data::Enum(syn::DataEnum {
    //         enum_token: Default::default(),
    //         brace_token: Default::default(),
    //         variants: Default::default(),
    //     });

    //     get_struct_data(&data, &name);
    // }

    // #[test]
    // #[should_panic(expected = "Must derive on a struct")]
    // fn test_get_struct_data_union() {
    //     let name = Ident::new("TestUnion", Span::call_site());
    //     let data = Data::Union(syn::DataUnion {
    //         union_token: Default::default(),
    //         fields: syn::FieldsNamed {
    //             brace_token: Default::default(),
    //             named: Default::default(),
    //         },
    //     });

    #[test]
    fn test_get_attributes_empty_struct() {
        let data_struct = DataStruct {
            fields: Fields::Named(FieldsNamed {
                named: Default::default(),
                brace_token: Default::default(),
            }),
            struct_token: Default::default(),
            semi_token: None,
        };

        let attributes = get_attributes(&data_struct);
        assert!(attributes.is_empty());
    }

    #[test]
    fn test_get_attributes_with_single_field() {
        let ident = format_ident!("field_name");
        let attr = Attribute::parse_outer.parse_str("#[serde]").unwrap();

        let field = Field {
            attrs: vec![attr[0].clone()],
            vis: syn::Visibility::Inherited,
            ident: Some(ident.clone()),
            colon_token: None,
            ty: syn::Type::Verbatim(proc_macro2::TokenStream::new()),
            mutability: syn::FieldMutability::None,
        };

        let data_struct = DataStruct {
            fields: Fields::Named(FieldsNamed {
                named: syn::punctuated::Punctuated::from_iter(vec![field]),
                brace_token: Default::default(),
            }),
            struct_token: Default::default(),
            semi_token: None,
        };

        let attributes = get_attributes(&data_struct);
        assert_eq!(attributes.len(), 1);
        assert_eq!(attributes.get("serde").unwrap().to_string(), "field_name");
    }

    #[test]
    fn test_get_attributes_multiple_fields() {
        let field1 = Field {
            attrs: vec![Attribute::parse_outer.parse_str("#[serde]").unwrap()[0].clone()],
            vis: syn::Visibility::Inherited,
            ident: Some(format_ident!("field1")),
            colon_token: None,
            ty: syn::Type::Verbatim(proc_macro2::TokenStream::new()),
            mutability: syn::FieldMutability::None,
        };

        let field2 = Field {
            attrs: vec![Attribute::parse_outer.parse_str("#[rename]").unwrap()[0].clone()],
            vis: syn::Visibility::Inherited,
            ident: Some(format_ident!("field2")),
            colon_token: None,
            ty: syn::Type::Verbatim(proc_macro2::TokenStream::new()),
            mutability: syn::FieldMutability::None,
        };

        let data_struct = DataStruct {
            fields: Fields::Named(FieldsNamed {
                named: syn::punctuated::Punctuated::from_iter(vec![field1, field2]),
                brace_token: Default::default(),
            }),
            struct_token: Default::default(),
            semi_token: None,
        };

        let attributes = get_attributes(&data_struct);
        assert_eq!(attributes.len(), 2);
        assert_eq!(attributes.get("serde").unwrap().to_string(), "field1");
        assert_eq!(attributes.get("rename").unwrap().to_string(), "field2");
    }

    #[test]
    fn test_get_attributes_no_attributes() {
        let field = Field {
            attrs: vec![],
            vis: syn::Visibility::Inherited,
            ident: Some(format_ident!("field1")),
            colon_token: None,
            ty: syn::Type::Verbatim(proc_macro2::TokenStream::new()),
            mutability: syn::FieldMutability::None,
        };

        let data_struct = DataStruct {
            fields: Fields::Named(FieldsNamed {
                named: syn::punctuated::Punctuated::from_iter(vec![field]),
                brace_token: Default::default(),
            }),
            struct_token: Default::default(),
            semi_token: None,
        };

        let attributes = get_attributes(&data_struct);
        assert!(attributes.is_empty());
    }
}
