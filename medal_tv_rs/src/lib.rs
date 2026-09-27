//! A Rust library for the medal.tv API.
//!
//! ## Example
//!
//! `MEDAL_TV_API_KEY=<api_key> MEDAL_TV_USER_ID=<user-id> cargo run --example get_all_clips`
//!
//! ```rust no_run
//! use medal_tv_rs::api::latest::latest_payload::LatestPayload;
//! use medal_tv_rs::api::latest::latest_response::LatestResponse;
//! use medal_tv_rs::error::MedalError;
//! use medal_tv_rs::medal::MedalClient;
//! use std::env;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), MedalError> {
//!     let medal_tv_api_key: String = env::var("MEDAL_TV_API_KEY").expect("no medal.tv API key");
//!     let medal_tv_user_id: String = env::var("MEDAL_TV_USER_ID").expect("no medal.tv user ID");
//!
//!     let medal_client: MedalClient = MedalClient::new(medal_tv_api_key, None);
//!     let latest_payload: LatestPayload = LatestPayload::builder().user_id(medal_tv_user_id).build();
//!     let latest_response: LatestResponse = medal_client.latest(&latest_payload).await?;
//!
//!     println!("{}", serde_json::to_string(&latest_response).unwrap());
//!     Ok(())
//! }
//! ```
//!
//! For more examples, check out the `examples` directory within the repository.

pub mod api;
pub mod error;
pub mod medal;
