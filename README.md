# yelp-fusion-rs

[![Version](https://img.shields.io/crates/v/medal-tv-rs)](https://crates.io/crates/medal-tv-rs)
[![Docs](https://docs.rs/medal-tv-rs/badge.svg)](https://docs.rs/medal-tv-rs)
[![License](https://img.shields.io/crates/l/medal-tv-rs)](https://crates.io/crates/medal-tv-rs)

A Rust library for the medal.tv API.

## Features

- [ ] [`/v1/generate_public_key` Generate Public-Use API Keys](https://docs.medal.tv/api#generate-an-api-key)
- [ ] [`/v1/generate_private_key` Generate Private-Use API Keys](https://docs.medal.tv/api#generate-an-api-key)
- [ ] [`/v1/trending` Trending Clips](https://docs.medal.tv/api#v1trending---trending-clips-by-game)
- [X] [`/v1/latest` Latest Clips (from a user or game)](https://docs.medal.tv/api#v1latest---latest-clips-from-a-user-or-game)
- [ ] [`/v1/search` Search Clips](https://docs.medal.tv/api#v1search---search-clips-on-medal)
- [ ] [`/v1/categories` Games List](https://docs.medal.tv/api#v1categories---games-list)

## Examples

Query every clip you've ever recorded!

`MEDAL_TV_API_KEY=<api_key> MEDAL_TV_USER_ID=<user-id> cargo run --example get_all_clips`

```rust
#[tokio::main]
async fn main() -> Result<(), MedalError> {
    let medal_tv_api_key: String = env::var("MEDAL_TV_API_KEY").expect("no medal.tv API key");
    let medal_tv_user_id: String = env::var("MEDAL_TV_USER_ID").expect("no medal.tv user ID");

    let medal_client: MedalClient = MedalClient::new(medal_tv_api_key, None);
    let latest_payload: LatestPayload = LatestPayload::builder().user_id(medal_tv_user_id).build();
    let latest_response: LatestResponse = medal_client.latest(&latest_payload).await?;

    println!("{}", serde_json::to_string(&latest_response).unwrap());
    Ok(())
}
```

For more examples, check out the [examples](https://github.com/goddtriffin/medal-tv-rs/blob/main/examples) directory.

## Developers

Project is under active maintenance - even if there are no recent commits!
Please submit an issue / bug request if you the library needs updating for any reason!

### Feature Requests

#### Implement the rest of the API endpoints.

Currently, I only have a use-case for medal.tv API's `/v1/latest` endpoint,
so I haven't prioritized developing the rest of endpoints.

I fully intend to implement all of those features so that this library can do everything the medal.tv API allows.

If you have a dire need for any of those endpoints, please ping me via an issue on GitHub and I'll know to prioritize that work.
If you're feeling extra adventurous and/or REALLY need those endpoints implemented, please send a pull request :)

### Commands

- `make lint`
    - Lints the codebase via `cargo fmt`.
- `make test`
    - Tests the codebase via:
        - `cargo fmt`
        - `cargo check`
        - `cargo clippy` (with insanely strict defaults)
        - `cargo test`.

## Credits

Made with 🤬 and 🥲 by [Todd Everett Griffin](https://www.toddgriffin.me/).

`medal-tv-rs` is open source under the [MIT License](https://github.com/goddtriffin/medal-tv-rs/blob/main/LICENSE).
