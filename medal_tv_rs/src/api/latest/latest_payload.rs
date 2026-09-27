use crate::api::latest::latest_payload_builder::LatestPayloadBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatestPayload {
    /// The user ID to search for.
    /// You can find your user ID by searching your profile on medal.tv and clicking on your profile.
    /// It’ll be medal.tv/users/`userId`.
    #[serde(rename = "userId")]
    user_id: Option<String>,

    /// Filter by game.
    /// Not sure what `categoryId` to look for?
    /// Just search for your game [here](https://jsoneditoronline.org/?url=https%3A%2F%2Fapi-v2.medal.tv%2Fcategories).
    #[serde(rename = "categoryId")]
    category_id: Option<String>,

    /// How many objects to return.
    /// By default you have access to 1000 objects per query.
    limit: Option<u16>,

    /// How many objects to skip.
    /// `limit` + `offset` can not exceed 1000 by default.
    offset: Option<u16>,
}

impl Default for LatestPayload {
    fn default() -> Self {
        Self {
            user_id: None,
            category_id: None,
            limit: Some(10),
            offset: Some(0),
        }
    }
}

impl LatestPayload {
    #[must_use]
    pub fn new(
        user_id: Option<String>,
        category_id: Option<String>,
        limit: Option<u16>,
        offset: Option<u16>,
    ) -> LatestPayload {
        Self {
            user_id,
            category_id,
            limit,
            offset,
        }
    }

    #[must_use]
    pub fn builder() -> LatestPayloadBuilder {
        LatestPayloadBuilder::default()
    }
}
