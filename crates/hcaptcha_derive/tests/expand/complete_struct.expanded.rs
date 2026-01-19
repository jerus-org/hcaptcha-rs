use hcaptcha_derive::Hcaptcha;
pub struct ContactForm {
    name: String,
    #[allow(dead_code)]
    phone: String,
    email: String,
    #[allow(dead_code)]
    message: String,
    #[captcha]
    token: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for ContactForm {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f,
            "ContactForm",
            "name",
            &self.name,
            "phone",
            &self.phone,
            "email",
            &self.email,
            "message",
            &self.message,
            "token",
            &&self.token,
        )
    }
}
impl Hcaptcha for ContactForm {
    fn valid_response(
        &self,
        secret: &str,
        uri: Option<String>,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<Output = Result<hcaptcha::Response, hcaptcha::Error>>,
        >,
    > {
        let mut client = hcaptcha::Client::new();
        if let Some(u) = uri {
            match client.set_url(&u) {
                Ok(c) => client = c,
                Err(e) => {
                    return Box::pin(async { Err(e) });
                }
            };
        }
        #[allow(unused_mut)]
        let mut captcha;
        match hcaptcha::Captcha::new(&self.token) {
            Ok(c) => captcha = c,
            Err(e) => {
                return Box::pin(async { Err(e) });
            }
        };
        let request;
        match hcaptcha::Request::new(&secret, captcha) {
            Ok(r) => request = r,
            Err(e) => {
                return Box::pin(async { Err(e) });
            }
        };
        Box::pin(client.verify(request))
    }
}
