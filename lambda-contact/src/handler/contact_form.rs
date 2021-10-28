use hcaptcha::Hcaptcha;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Default, Hcaptcha)]
pub struct ContactForm {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub message: String,
    pub page: String,
    pub site: String,
    #[captcha]
    pub response: String,
    #[remoteip]
    pub remoteip: String,
    #[sitekey]
    pub sitekey: String,
}

impl ContactForm {
    pub(crate) fn page_name(&self) -> String {
        self.page.replace("/", "")
    }

    pub(crate) fn site_name(&self) -> String {
        let mut domain_page = self.site.replace("https://", "");

        tracing::debug!("The domain_page is: {}", domain_page);
        let boundary = domain_page.find('/').unwrap_or(0);
        if boundary > 0 {
            let page = domain_page.split_off(boundary);
            tracing::debug!(
                "After split page = {} and domain_page = {}",
                page,
                domain_page
            );
            return domain_page;
        }

        "domain not found".to_owned()
    }
}
