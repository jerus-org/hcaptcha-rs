use crate::Request;

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct Form {
    response: String,
    remoteip: Option<String>,
    sitekey: Option<String>,
    secret: String,
}

impl From<Request> for Form {
    fn from(request: Request) -> Form {
        let remoteip = request.captcha().remoteip.map(|v| v.to_string());
        let sitekey = request.captcha().sitekey.map(|v| v.to_string());

        Form {
            response: request.captcha().response.to_string(),
            remoteip,
            sitekey,
            secret: request.secret().to_string(),
        }
    }
}
