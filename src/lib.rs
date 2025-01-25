mod nostr;
mod tests;
use nostro2::notes::{NostrNote, NostrTag, NoteTags, TagList};
use serde::{Deserialize, Serialize};

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Empty {}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct LoginRole {
    #[prost(bool, tag = "1")]
    pub is_admin: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Serialize, Deserialize, ::prost::Message)]
pub struct GrpcNostrNoteId {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Serialize, Deserialize, ::prost::Message)]
pub struct GrpcNostrNoteTag {
    #[prost(string, repeated, tag = "1")]
    pub tag: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl GrpcNostrNoteTag {
    pub fn to_vec(&self) -> Vec<String> {
        self.tag.clone()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Serialize, Deserialize, ::prost::Message)]
pub struct GrpcNostrNote {
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub kind: u32,
    #[prost(string, tag = "3")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub created_at: i64,
    #[prost(message, repeated, tag = "5")]
    pub tags: ::prost::alloc::vec::Vec<GrpcNostrNoteTag>,
    #[prost(string, tag = "6")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub sig: ::prost::alloc::string::String,
}
impl From<NostrNote> for GrpcNostrNote {
    fn from(note: NostrNote) -> Self {
        let tags = note
            .tags
            .0
            .iter()
            .map(|tag| GrpcNostrNoteTag {
                tag: {
                    let mut tags = vec![];
                    match tag.tag_type {
                        NostrTag::Pubkey => tags.push("p".to_string()),
                        NostrTag::Parameterized => tags.push("d".to_string()),
                        NostrTag::Event => tags.push("e".to_string()),
                        NostrTag::Custom(custom) => tags.push(custom.to_string()),
                    }
                    tags.extend(tag.tags.clone());
                    tags
                },
            })
            .collect();
        GrpcNostrNote {
            content: note.content.clone(),
            kind: note.kind,
            pubkey: note.pubkey.clone(),
            created_at: note.created_at,
            tags,
            id: note.id.clone().unwrap_or_default(),
            sig: note.sig.clone().unwrap_or_default(),
        }
    }
}
impl Into<NostrNote> for GrpcNostrNote {
    fn into(self) -> NostrNote {
        let tags = self
            .tags
            .iter()
            .filter_map(|tag| {
                let tag_list = TagList {
                    tag_type: {
                        let first = tag.tag.first()?;
                        match first.as_str() {
                            "p" => NostrTag::Pubkey,
                            "d" => NostrTag::Parameterized,
                            "e" => NostrTag::Event,
                            other => {
                                NostrTag::Custom(Box::leak(other.to_string().into_boxed_str()))
                            }
                        }
                    },
                    tags: tag.tag.iter().skip(1).map(|tag| tag.clone()).collect(),
                };
                Some(tag_list)
            })
            .collect::<Vec<TagList>>();
        NostrNote {
            pubkey: self.pubkey,
            kind: self.kind,
            content: self.content,
            created_at: self.created_at,
            tags: NoteTags(tags),
            id: Some(self.id),
            sig: Some(self.sig),
        }
    }
}
impl TryFrom<String> for GrpcNostrNote {
    type Error = serde_json::Error;
    fn try_from(json: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&json)
    }
}
impl TryFrom<&str> for GrpcNostrNote {
    type Error = serde_json::Error;
    fn try_from(json: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(json)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Serialize, Deserialize, ::prost::Message)]
pub struct GrpcNostrNoteList {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<GrpcNostrNote>,
}
impl GrpcNostrNoteList {
    pub fn to_signed_note_vec(&self) -> Vec<NostrNote> {
        self.events
            .iter()
            .map(|event| (*event).clone().into())
            .collect()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    #[prost(string, repeated, tag = "1")]
    pub authors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, repeated, tag = "2")]
    pub kinds: ::prost::alloc::vec::Vec<u32>,
    #[prost(string, repeated, tag = "3")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag = "4")]
    pub since: u32,
    #[prost(uint32, tag = "5")]
    pub until: u32,
    #[prost(uint32, tag = "6")]
    pub limit: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterTag {
    #[prost(string, repeated, tag = "1")]
    pub parameter: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
