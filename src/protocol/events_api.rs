//! Events API JSON Types

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Event<'s> {
    #[serde(borrow = "'s")]
    Message(MessageEvent<'s>),
    #[serde(borrow = "'s")]
    ReactionAdded(ReactionEvent<'s>),
    #[serde(borrow = "'s")]
    ReactionRemoved(ReactionEvent<'s>),
}

#[derive(Deserialize, Debug)]
pub struct MessageEvent<'s> {
    pub event_ts: &'s str,
    pub subtype: Option<&'s str>,
    pub text: Option<std::borrow::Cow<'s, str>>,
    pub user: Option<&'s str>,
    pub ts: Option<&'s str>,
    pub deleted_ts: Option<&'s str>,
    pub team: Option<&'s str>,
    pub channel: &'s str,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub is_starred: bool,
    #[serde(default)]
    pub pinned_to: Vec<&'s str>,
    #[serde(default, borrow = "'s")]
    pub reactions: Vec<MessageReaction<'s>>,
}

#[derive(Deserialize, Debug)]
pub struct MessageReaction<'s> {
    pub name: &'s str,
    pub count: u32,
    #[serde(default, borrow = "'s")]
    pub users: Vec<&'s str>,
}

#[derive(Deserialize, Debug)]
pub struct ReactionEvent<'s> {
    pub event_ts: &'s str,
    pub user: &'s str,
    pub reaction: &'s str,
    pub item_user: Option<&'s str>,
    #[serde(borrow = "'s")]
    pub item: ReactionItem<'s>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ReactionItem<'s> {
    Message { channel: &'s str, ts: &'s str },
    File { file: &'s str },
    FileComment { file_comment: &'s str, file: &'s str },
}
