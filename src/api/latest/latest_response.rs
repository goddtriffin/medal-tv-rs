use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatestResponse {
    content_objects: Vec<ContentObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentObject {
    /// The unique identifier for the content
    #[serde(rename = "contentId")]
    content_id: String,

    /// The URL for the raw file, if authorized
    #[serde(rename = "rawFileUrl")]
    raw_file_url: String,

    /// The title of the content
    #[serde(rename = "contentTitle")]
    content_title: String,

    /// The number of views the content has received
    #[serde(rename = "contentViews")]
    content_views: u32,

    /// The number of likes the content has received
    #[serde(rename = "contentLikes")]
    content_likes: u32,

    /// The category identifier for the content
    #[serde(rename = "categoryId")]
    category_id: u32,

    /// The length of the video in seconds
    #[serde(rename = "videoLengthSeconds")]
    video_length_seconds: u32,

    /// The timestamp when the content was created
    #[serde(rename = "createdTimestamp")]
    created_timestamp: u64,

    /// The direct URL for the clip
    #[serde(rename = "directClipUrl")]
    direct_clip_url: String,

    /// The URL for the embeddable iframe
    #[serde(rename = "embedIframeUrl")]
    embed_iframe_url: String,

    /// The credits for the content
    credits: String,
}
