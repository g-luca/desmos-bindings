//! Contains structs and enums related to the x/posts module.

use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Contains all the information about a single post.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Post {
    /// Unique id of the post.
    pub id: Uint64,
    /// Id of the subspace inside which the post has been created.
    pub subspace_id: Uint64,
    /// Id of the section inside which the post has been created.
    pub section_id: u32,
    /// External id for this post.
    pub eternal_id: Option<String>,
    /// Text of the post.
    pub text: Option<String>,
    /// Entities connected to this post.
    pub entities: Option<Vec<Entities>>,
    /// Author of the post.
    pub author: Addr,
    /// Id of the original post of the conversation.
    pub conversation_id: Option<Uint64>,
    /// A list this posts references (either as a reply, repost or quote).
    pub referenced_posts: Vec<PostReference>,
    /// Reply settings of this post.
    pub reply_settings: ReplySetting,
    /// Creation date of the post in RFC 3339 format.
    /// example: 1972-01-01T10:00:20.
    pub creation_date: String,
    /// Last edited time of the post in RFC 3339 format.
    /// example: 1972-01-01T10:00:20.
    pub last_edit_date: Option<String>,
}

/// Represents a generic tag.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TagEntity {
    /// Index of the character inside the text at which the tag starts.
    pub start: Uint64,
    /// Index of the character inside the text at which the tag ends.
    pub end: Uint64,
    /// Tag reference (user address, hashtag value, etc).
    pub tag: String,
}

/// Contains the details of a generic URL.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UrlEntity {
    /// Index of the character inside the text at which the URL starts.
    pub start: Uint64,
    /// Index of the character inside the text at which the URL ends.
    pub end: Uint64,
    /// Value of the URL where the user should be redirected to.
    pub url: String,
    /// Display value of the URL.
    pub display_url: Option<String>,
}

/// Contains the details of entities parsed out of the post text.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Entities {
    /// Hashtag entities.
    pub hashtags: Vec<TagEntity>,
    /// Mention entities.
    pub mentions: Vec<TagEntity>,
    /// Url entities.
    pub urls: Vec<UrlEntity>,
}

/// Contains the details of a post reference.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PostReference {
    /// Type of reference.
    #[serde(rename = "type")]
    pub ref_type: PostReferenceType,
    /// Id of the referenced post.
    pub post_id: Uint64,
    /// Position of the reference inside the post's text. This should be used only
    /// with the type set to ['PostReferenceType::Quote']
    pub position: Option<Uint64>,
}

/// Represents the different types of [`PostReference`].
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PostReferenceType {
    /// No reference specified.
    Unspecified,
    /// Represents a reply to the specified post.
    ReplayTo,
    /// Represents a quote of the specified post.
    Quote,
    /// Represents a repost of the specified post.
    Repost,
}

/// Contains the possible reply settings that a post can have.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReplySetting {
    /// No reply setting specified.
    #[serde(rename = "REPLY_SETTING_UNSPECIFIED")]
    Unspecified,
    /// Everyone will be able to reply to this post.
    #[serde(rename = "REPLY_SETTING_EVERYONE")]
    Everyone,
    /// Only followers of the author will be able to reply to this post.
    #[serde(rename = "REPLY_SETTING_FOLLOWERS")]
    Followers,
    /// Only the author mutual followers will be able to reply to this post.
    #[serde(rename = "REPLY_SETTING_MUTUAL")]
    Mutual,
    /// Only people mentioned inside this post will be able to reply.
    #[serde(rename = "REPLY_SETTING_MENTIONS")]
    Mentions,
}

/// Contains the data of a single post attachment.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Attachment {
    /// Id of the subspace inside which the post to which this attachment should be
    /// connected is.
    pub subspace_id: Uint64,
    /// Id of the subspace section inside which the post to which this attachment
    /// should be connected is.
    pub section_id: u32,
    /// Id of the post to which this attachment should be connected.
    pub post_id: Uint64,
    /// Id of this attachment.
    pub id: u32,
    /// Content of the attachment.
    pub content: PostAttachment,
}

/// Contains the result of a single poll provided answer.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AnswerResult {
    /// Index of the answer inside the [`PostAttachment::Poll::provided_answers`].
    pub answer_index: u32,
    /// Number of votes the answer has received.
    pub votes: Uint64,
}

/// Contains the tally results for a [`PostAttachment::Poll`].
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PollTallyResults {
    /// Lists of votes.
    pub results: Vec<AnswerResult>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// Contains the details of a possible poll answer
pub struct ProvidedAnswer {
    /// Text of the answer.
    pub text: Option<String>,
    /// Attachments of the answer.
    pub attachments: Vec<Attachment>,
}

/// Represents a user answer to a poll.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UserAnswer {
    /// Subspace id inside which the post related to this attachment is located.
    pub subspace_id: Uint64,
    /// Section id inside which the post related to this attachment is located.
    pub section_id: u32,
    /// Id of the post associated to this attachment.
    pub post_id: Uint64,
    /// Id of the poll to which this answer is associated.
    pub poll_id: u32,
    /// Indexes of the answers inside the [`PostAttachment::Poll::provided_answers`] array.
    pub answers_indexes: Vec<u32>,
    /// Address of the user answering the poll.
    pub user: Addr,
}

/// Supported attachment that can be attached to a post.
#[derive(Clone, Debug, PartialEq, JsonSchema)]
pub enum PostAttachment {
    /// Represents a media attachment.
    Media {
        /// Media mime type.
        mime_type: String,
        /// Uri where can be found the media.
        uri: String,
    },
    /// Represents a poll attachment.
    Poll {
        /// Question of the poll.
        question: String,
        /// Answers the users can choose from.
        provided_answers: Vec<ProvidedAnswer>,
        /// Date at which the poll will close in RFC 3339 format.
        /// example: 1972-01-01T10:00:20.
        end_date: String,
        /// Whether the poll allows multiple choices from the same user or not.
        allows_multiple_answers: bool,
        /// Whether the poll allows to edit an answer or not.
        allows_answer_edits: bool,
        /// Final poll results.
        final_tally_results: Option<PollTallyResults>,
    },
}
