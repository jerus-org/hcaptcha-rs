extern crate proc_macro;

use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::DeriveInput;

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

    let data_struct = match data {
        syn::Data::Struct(s) => s,
        _ => {
            let example = r#"
        #[derive(Hcaptcha)]
        struct MyStruct {
            #[captcha]
            hcaptcha: String,
        }"#;
            proc_macro_error::abort! {name,
                "Must derive on a struct";
                help = "This macro can only be implemented on a struct.
                {}", &example;
            };
        }
    };

    let mut attributes = HashMap::new();

    let _: Vec<String> = data_struct
        .fields
        .iter()
        .filter(|f| !f.attrs.is_empty())
        .map(|f| {
            let a = f.attrs[0].path.get_ident().unwrap().to_string();
            if let Some(i) = &f.ident {
                // let n = i.to_string();
                attributes.insert(a.clone(), i);
            }
            a
        })
        .collect();

    let captcha = get_required_attribute(&attributes, "captcha", name);

    let remoteip = get_optional_attribute(&attributes, "remoteip", "set_remoteip");
    let sitekey = get_optional_attribute(&attributes, "sitekey", "set_sitekey");

    let gen = quote! {
        impl #impl_generics Hcaptcha for #name #ty_generics #where_clause {
            fn valid_response(&self, secret: &str, uri: Option<String>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<hcaptcha::HcaptchaResponse, hcaptcha::HcaptchaError>> + Send>>  {
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
/// Error if not found
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
            proc_macro_error::abort! {id,
                "Field containing hcaptcha not identified";
                help = "The field containing the hcaptcha response string must be identified with #[captcha]
                {}", &example;
            };
        }
    }
}
