use crate::posts::models::{PollTallyResults, PostAttachment, ProvidedAnswer};
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;

const MEDIA_TYPE_URI: &str = "/desmos.posts.v1.Media";
const POLL_TYPE_URI: &str = "/desmos.posts.v1.Poll";

impl Serialize for PostAttachment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            PostAttachment::Media { mime_type, uri } => {
                let mut sv = serializer.serialize_struct("PostAttachment::Media", 3)?;
                sv.serialize_field("@type", MEDIA_TYPE_URI)?;
                sv.serialize_field("mime_type", mime_type)?;
                sv.serialize_field("uri", uri)?;
                sv.end()
            }
            PostAttachment::Poll {
                question,
                provided_answers,
                end_date,
                allows_multiple_answers,
                allows_answer_edits,
                final_tally_results,
            } => {
                let mut sv = serializer.serialize_struct("PostAttachment::Poll", 7)?;
                sv.serialize_field("@type", POLL_TYPE_URI)?;
                sv.serialize_field("question", question)?;
                sv.serialize_field("provided_answers", provided_answers)?;
                sv.serialize_field("end_date", end_date)?;
                sv.serialize_field("allows_multiple_answers", allows_multiple_answers)?;
                sv.serialize_field("allows_answer_edits", allows_answer_edits)?;
                sv.serialize_field("final_tally_results", final_tally_results)?;
                sv.end()
            }
        }
    }
}

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum Field {
    #[serde(rename = "@type")]
    TypeUri,
    MimeType,
    Uri,
    Question,
    ProvidedAnswers,
    EndDate,
    AllowsMultipleAnswers,
    AllowsAnswerEdits,
    FinalTallyResults,
}

struct PostAttachmentVisitor;

impl<'de> Visitor<'de> for PostAttachmentVisitor {
    type Value = PostAttachment;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("enum PostAttachment")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut type_uri: Option<String> = None;
        let mut mime_type: Option<String> = None;
        let mut uri: Option<String> = None;
        let mut question: Option<String> = None;
        let mut provided_answers: Option<Vec<ProvidedAnswer>> = None;
        let mut end_date: Option<String> = None;
        let mut allows_multiple_answers: Option<bool> = None;
        let mut allows_answer_edits: Option<bool> = None;
        let mut final_tally_results: Option<Option<PollTallyResults>> = None;

        while let Some(key) = map.next_key()? {
            match key {
                Field::TypeUri => {
                    if type_uri.is_some() {
                        return Err(de::Error::duplicate_field("@type"));
                    }
                    type_uri = Some(map.next_value()?);
                }
                Field::MimeType => {
                    if mime_type.is_some() {
                        return Err(de::Error::duplicate_field("mime_type"));
                    }
                    mime_type = Some(map.next_value()?);
                }
                Field::Uri => {
                    if uri.is_some() {
                        return Err(de::Error::duplicate_field("uri"));
                    }
                    uri = Some(map.next_value()?);
                }
                Field::Question => {
                    if question.is_some() {
                        return Err(de::Error::duplicate_field("question"));
                    }
                    question = Some(map.next_value()?);
                }
                Field::ProvidedAnswers => {
                    if provided_answers.is_some() {
                        return Err(de::Error::duplicate_field("provided_answers"));
                    }
                    provided_answers = Some(map.next_value()?);
                }
                Field::EndDate => {
                    if end_date.is_some() {
                        return Err(de::Error::duplicate_field("end_date"));
                    }
                    end_date = Some(map.next_value()?);
                }
                Field::AllowsMultipleAnswers => {
                    if allows_multiple_answers.is_some() {
                        return Err(de::Error::duplicate_field("allows_multiple_answers"));
                    }
                    allows_multiple_answers = Some(map.next_value()?);
                }
                Field::AllowsAnswerEdits => {
                    if allows_answer_edits.is_some() {
                        return Err(de::Error::duplicate_field("allows_answer_edits"));
                    }
                    allows_answer_edits = Some(map.next_value()?);
                }
                Field::FinalTallyResults => {
                    if final_tally_results.is_some() {
                        return Err(de::Error::duplicate_field("final_tally_results"));
                    }
                    final_tally_results = Some(map.next_value()?);
                }
            }
        }

        let type_uri: String = type_uri.ok_or_else(|| de::Error::missing_field("@type"))?;

        if type_uri == MEDIA_TYPE_URI {
            let mime_type = mime_type.ok_or_else(|| de::Error::missing_field("mime_type"))?;
            let uri = uri.ok_or_else(|| de::Error::missing_field("uri"))?;
            Ok(PostAttachment::Media { mime_type, uri })
        } else if type_uri == POLL_TYPE_URI {
            let question = question.ok_or_else(|| de::Error::missing_field("question"))?;
            let provided_answers =
                provided_answers.ok_or_else(|| de::Error::missing_field("provided_answers"))?;
            let end_date = end_date.ok_or_else(|| de::Error::missing_field("end_date"))?;
            let allows_multiple_answers = allows_multiple_answers
                .ok_or_else(|| de::Error::missing_field("allows_multiple_answers"))?;
            let allows_answer_edits = allows_answer_edits
                .ok_or_else(|| de::Error::missing_field("allows_answer_edits"))?;
            let final_tally_results =
                final_tally_results.and_then(|poll_tally_result| poll_tally_result);
            Ok(PostAttachment::Poll {
                question,
                provided_answers,
                end_date,
                allows_multiple_answers,
                allows_answer_edits,
                final_tally_results,
            })
        } else {
            Err(de::Error::missing_field("@type"))
        }
    }
}

impl<'de> Deserialize<'de> for PostAttachment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "@type",
            "mime_type",
            "uri",
            "question",
            "provided_answers",
            "end_date",
            "allows_multiple_answers",
            "allows_answer_edits",
            "final_tally_results",
        ];
        deserializer.deserialize_struct("PostAttachment", FIELDS, PostAttachmentVisitor)
    }
}

#[cfg(test)]
mod test {
    use crate::posts::models::{
        AnswerResult, Attachment, PollTallyResults, PostAttachment, ProvidedAnswer,
    };
    use cosmwasm_std::Uint64;

    #[test]
    fn test_media_attachment_serialization() {
        let media = PostAttachment::Media {
            mime_type: "test_mime".to_string(),
            uri: "test_uri".to_string(),
        };
        let serialized = serde_json::to_string(&media).unwrap();

        assert_eq!("{\"@type\":\"/desmos.posts.v1.Media\",\"mime_type\":\"test_mime\",\"uri\":\"test_uri\"}", &serialized);
    }

    #[test]
    fn test_media_attachment_deserialization() {
        let expected = PostAttachment::Media {
            mime_type: "test_mime".to_string(),
            uri: "test_uri".to_string(),
        };
        let serialized = "{\"@type\":\"/desmos.posts.v1.Media\",\"mime_type\":\"test_mime\",\"uri\":\"test_uri\"}";

        let deserialized: PostAttachment = serde_json::from_str(serialized).unwrap();

        assert_eq!(expected, deserialized);
    }

    #[test]
    fn test_poll_attachment_serialization() {
        let poll = PostAttachment::Poll {
            question: "poll question".to_string(),
            provided_answers: vec![ProvidedAnswer {
                text: Some("Question 1".to_string()),
                attachments: vec![Attachment {
                    subspace_id: Default::default(),
                    section_id: 0,
                    post_id: Default::default(),
                    id: 0,
                    content: PostAttachment::Media {
                        mime_type: "test_mime".to_string(),
                        uri: "test_uri".to_string(),
                    },
                }],
            }],
            end_date: "1234".to_string(),
            allows_multiple_answers: false,
            allows_answer_edits: false,
            final_tally_results: Some(PollTallyResults {
                results: vec![AnswerResult {
                    answer_index: 1,
                    votes: Uint64::new(1),
                }],
            }),
        };
        let serialized = serde_json::to_string(&poll).unwrap();
        assert_eq!("{\"@type\":\"/desmos.posts.v1.Poll\",\"question\":\"poll question\",\"provided_answers\":[{\"text\":\"Question 1\",\"attachments\":[{\"subspace_id\":\"0\",\"section_id\":0,\"post_id\":\"0\",\"id\":0,\"content\":{\"@type\":\"/desmos.posts.v1.Media\",\"mime_type\":\"test_mime\",\"uri\":\"test_uri\"}}]}],\"end_date\":\"1234\",\"allows_multiple_answers\":false,\"allows_answer_edits\":false,\"final_tally_results\":{\"results\":[{\"answer_index\":1,\"votes\":\"1\"}]}}", &serialized);

        let poll = PostAttachment::Poll {
            question: "poll question".to_string(),
            provided_answers: vec![ProvidedAnswer {
                text: Some("Question 1".to_string()),
                attachments: vec![Attachment {
                    subspace_id: Default::default(),
                    section_id: 0,
                    post_id: Default::default(),
                    id: 0,
                    content: PostAttachment::Media {
                        mime_type: "test_mime".to_string(),
                        uri: "test_uri".to_string(),
                    },
                }],
            }],
            end_date: "1234".to_string(),
            allows_multiple_answers: false,
            allows_answer_edits: false,
            final_tally_results: None,
        };

        let serialized = serde_json::to_string(&poll).unwrap();
        assert_eq!("{\"@type\":\"/desmos.posts.v1.Poll\",\"question\":\"poll question\",\"provided_answers\":[{\"text\":\"Question 1\",\"attachments\":[{\"subspace_id\":\"0\",\"section_id\":0,\"post_id\":\"0\",\"id\":0,\"content\":{\"@type\":\"/desmos.posts.v1.Media\",\"mime_type\":\"test_mime\",\"uri\":\"test_uri\"}}]}],\"end_date\":\"1234\",\"allows_multiple_answers\":false,\"allows_answer_edits\":false,\"final_tally_results\":null}", &serialized);
    }

    #[test]
    fn test_poll_attachment_deserialization() {
        let serialized = "{\"@type\":\"/desmos.posts.v1.Poll\",\"question\":\"poll question\",\"provided_answers\":[{\"text\":\"Question 1\",\"attachments\":[{\"subspace_id\":\"0\",\"section_id\":0,\"post_id\":\"0\",\"id\":0,\"content\":{\"@type\":\"/desmos.posts.v1.Media\",\"mime_type\":\"test_mime\",\"uri\":\"test_uri\"}}]}],\"end_date\":\"1234\",\"allows_multiple_answers\":false,\"allows_answer_edits\":false,\"final_tally_results\":{\"results\":[{\"answer_index\":1,\"votes\":\"1\"}]}}";
        let deserialized: PostAttachment = serde_json::from_str(serialized).unwrap();
        assert_eq!(
            PostAttachment::Poll {
                question: "poll question".to_string(),
                provided_answers: vec![ProvidedAnswer {
                    text: Some("Question 1".to_string()),
                    attachments: vec![Attachment {
                        subspace_id: Default::default(),
                        section_id: 0,
                        post_id: Default::default(),
                        id: 0,
                        content: PostAttachment::Media {
                            mime_type: "test_mime".to_string(),
                            uri: "test_uri".to_string(),
                        },
                    }],
                }],
                end_date: "1234".to_string(),
                allows_multiple_answers: false,
                allows_answer_edits: false,
                final_tally_results: Some(PollTallyResults {
                    results: vec![AnswerResult {
                        answer_index: 1,
                        votes: Uint64::new(1),
                    }],
                }),
            },
            deserialized
        );

        let serialized = "{\"@type\":\"/desmos.posts.v1.Poll\",\"question\":\"poll question\",\"provided_answers\":[{\"text\":\"Question 1\",\"attachments\":[{\"subspace_id\":\"0\",\"section_id\":0,\"post_id\":\"0\",\"id\":0,\"content\":{\"@type\":\"/desmos.posts.v1.Media\",\"mime_type\":\"test_mime\",\"uri\":\"test_uri\"}}]}],\"end_date\":\"1234\",\"allows_multiple_answers\":false,\"allows_answer_edits\":false,\"final_tally_results\":null}";
        let deserialized: PostAttachment = serde_json::from_str(serialized).unwrap();
        assert_eq!(
            PostAttachment::Poll {
                question: "poll question".to_string(),
                provided_answers: vec![ProvidedAnswer {
                    text: Some("Question 1".to_string()),
                    attachments: vec![Attachment {
                        subspace_id: Default::default(),
                        section_id: 0,
                        post_id: Default::default(),
                        id: 0,
                        content: PostAttachment::Media {
                            mime_type: "test_mime".to_string(),
                            uri: "test_uri".to_string(),
                        },
                    }],
                }],
                end_date: "1234".to_string(),
                allows_multiple_answers: false,
                allows_answer_edits: false,
                final_tally_results: None,
            },
            deserialized
        );
    }
}
