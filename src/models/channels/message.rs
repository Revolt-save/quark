use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{models::attachment::File, types::january::Embed};

/// Representation of a system event message
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum SystemMessage {
    #[serde(rename = "text")]
    Text { content: String },
    #[serde(rename = "user_added")]
    UserAdded { id: String, by: String },
    #[serde(rename = "user_remove")]
    UserRemove { id: String, by: String },
    #[serde(rename = "user_joined")]
    UserJoined { id: String },
    #[serde(rename = "user_left")]
    UserLeft { id: String },
    #[serde(rename = "user_kicked")]
    UserKicked { id: String },
    #[serde(rename = "user_banned")]
    UserBanned { id: String },
    #[serde(rename = "channel_renamed")]
    ChannelRenamed { name: String, by: String },
    #[serde(rename = "channel_description_changed")]
    ChannelDescriptionChanged { by: String },
    #[serde(rename = "channel_icon_changed")]
    ChannelIconChanged { by: String },
}

/// Untagged enum representing message content
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Content {
    /// Message contains text content
    Text(String),
    /// Message is a system event
    SystemMessage(SystemMessage),
}

/// Name and / or avatar override information
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct Masquerade {
    /// Replace the display name shown on this message
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 32))]
    name: Option<String>,
    /// Replace the avatar shown on this message (URL to image file)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 128))]
    avatar: Option<String>,
}

/// Representation of a Message on Revolt
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: String,
    /// Unique value generated by client sending this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    /// Id of the channel this message was sent in
    pub channel: String,
    /// Id of the user that sent this message
    pub author: String,

    /// Message content
    pub content: Content,
    /// Array of attachments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<File>>,
    /// Time at which this message was last edited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited: Option<DateTime>,
    /// Attached embeds to this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Embed>>,
    /// Array of user ids mentioned in this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<String>>,
    /// Array of message ids this message is replying to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<String>>,
    /// Name and / or avatar overrides for this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masquerade: Option<Masquerade>,
}
