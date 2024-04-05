use reqwest::Client;

pub const BASE_URL: &str = "https://developers.medal.tv/v1";

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub struct MedalClient {
    pub(crate) client: Client,
    pub(crate) base_url: String,
    pub(crate) api_key: String,
}

impl MedalClient {
    #[must_use]
    pub fn new(api_key: String, client: Option<Client>) -> Self {
        Self {
            client: client.unwrap_or_default(),
            base_url: BASE_URL.to_string(),
            api_key,
        }
    }
}
