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
//!                     Output = Result<hcaptcha::HcaptchaResponse,
//!                                     hcaptcha::HcaptchaError>,
//!                                     > + Send,
//!         >,
//!     > {
//!         let mut client = hcaptcha::HcaptchaClient::new();
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
//!         match hcaptcha::HcaptchaCaptcha::new(&self.hcaptcha) {
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
//!         match hcaptcha::HcaptchaRequest::new(&secret, captcha) {
//!             Ok(r) => request = r,
//!             Err(e) => {
//!                 return Box::pin(async { Err(e) });
//!             }
//!         };
//!         Box::pin(client.verify_client_response(request))
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

#[cfg(not(target_arch = "wasm32"))]
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
            fn valid_response(&self, secret: &str, uri: Option<String>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<hcaptcha::HcaptchaResponse, hcaptcha::HcaptchaError>> >>  {
                let mut client = hcaptcha::HcaptchaClient::new();
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
                match hcaptcha::HcaptchaRequest::new(&secret, captcha) {
                    Ok(r) => request = r,
                    Err(e) => {
                        return Box::pin(async { Err(e) } );
                    }
                };
                Box::pin(client.verify_client_response(request))
            }
        }
    };
    gen.into()
}

#[cfg(target_arch = "wasm32")]
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
            fn valid_response(&self, secret: &str, uri: Option<String>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<hcaptcha::HcaptchaResponse, hcaptcha::HcaptchaError>> >>  {
                let mut client = hcaptcha::HcaptchaClient::new();
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
                match hcaptcha::HcaptchaRequest::new(&secret, captcha) {
                    Ok(r) => request = r,
                    Err(e) => {
                        return Box::pin(async { Err(e) } );
                    }
                };
                Box::pin(client.verify_client_response(request))
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
                match hcaptcha::HcaptchaCaptcha::new(&self.#i) {
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
