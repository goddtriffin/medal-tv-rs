use crate::api::latest::latest_payload::LatestPayload;

#[derive(Debug, Clone, Default)]
pub struct LatestPayloadBuilder {
    /// The user ID to search for.
    /// You can find your user ID by searching your profile on medal.tv and clicking on your profile.
    /// It’ll be medal.tv/users/`userId`.
    user_id: Option<String>,

    /// Filter by game.
    /// Not sure what `categoryId` to look for?
    /// Just search for your game [here](https://jsoneditoronline.org/?url=https%3A%2F%2Fapi-v2.medal.tv%2Fcategories).
    category_id: Option<String>,

    /// How many objects to return.
    /// By default you have access to 1000 objects per query.
    limit: Option<u16>,

    /// How many objects to skip.
    /// `limit` + `offset` can not exceed 1000 by default.
    offset: Option<u16>,
}

impl LatestPayloadBuilder {
    #[must_use]
    pub fn user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    #[must_use]
    pub fn category_id(mut self, category_id: String) -> Self {
        self.category_id = Some(category_id);
        self
    }

    #[must_use]
    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit);
        self
    }

    #[must_use]
    pub fn offset(mut self, offset: u16) -> Self {
        self.offset = Some(offset);
        self
    }

    #[must_use]
    pub fn build(self) -> LatestPayload {
        LatestPayload::new(self.user_id, self.category_id, self.limit, self.offset)
    }
}
