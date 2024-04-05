#[allow(clippy::module_name_repetitions)]
pub mod latest_payload;
#[allow(clippy::module_name_repetitions)]
pub mod latest_payload_builder;
#[allow(clippy::module_name_repetitions)]
pub mod latest_response;

use crate::api::latest::latest_payload::LatestPayload;
use crate::api::latest::latest_response::LatestResponse;
use crate::error::MedalError;
use crate::medal::MedalClient;
use bytes::Bytes;
use reqwest::{RequestBuilder, Response, StatusCode};

impl MedalClient {
    /// # Errors
    ///
    /// Will return `Err` if the request fails or the response is not valid JSON.
    pub async fn latest(&self, payload: &LatestPayload) -> Result<LatestResponse, MedalError> {
        // create request
        let request: RequestBuilder = self
            .client
            .get(format!("{}/latest", self.base_url))
            .header("Content-Type", "application/json")
            .header("Authorization", self.api_key.clone())
            .query(&payload);

        // send request, get response
        let response: Response = request.json(&payload).send().await?;

        // parse status code and returned bytes
        let status_code: StatusCode = response.status();
        let bytes: Bytes = response.bytes().await?;

        // check if failure
        if !status_code.is_success() {
            return Err(MedalError::RequestFailed { bytes, status_code });
        }

        // success
        let latest_response: LatestResponse = serde_json::from_slice(&bytes)?;
        Ok(latest_response)
    }
}
