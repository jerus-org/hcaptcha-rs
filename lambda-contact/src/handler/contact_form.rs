

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct ContactForm {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub phone: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub page: String,
    #[serde(default)]
    pub site: String,
}
impl ContactForm {
    fn page_name(&self) -> String {
        self.page.replace("/", "")
    }

    fn site_name(&self) -> String {
        let mut domain_page = self.site.replace("https://", "");

        debug!("The domain_page is: {}", domain_page);
        let boundary = domain_page.find('/').unwrap_or(0);
        if boundary > 0 {
            let page = domain_page.split_off(boundary);
            debug!(
                "After split page = {} and domain_page = {}",
                page, domain_page
            );
            return domain_page;
        }

        "domain not found".to_owned()
    }
}