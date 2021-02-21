//! JSON Types

use serde::{Deserialize, Serialize};
pub mod events_api;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Message<'s> {
    Hello {
        num_connections: u32,
        #[serde(borrow = "'s")]
        connection_info: ConnectionInfo<'s>,
        #[serde(borrow = "'s")]
        debug_info: DebugInfo<'s>,
    },
    Disconnect {
        reason: &'s str,
        #[serde(borrow = "'s")]
        debug_info: DebugInfo<'s>,
    },
    EventsApi {
        envelope_id: &'s str,
        #[serde(borrow = "'s")]
        payload: EventsApiPayload<'s>,
    },
}

#[derive(Serialize)]
pub struct Acknowledge<'s> {
    pub envelope_id: &'s str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<&'s str>,
}

#[derive(Deserialize, Debug)]
pub struct ConnectionInfo<'s> {
    pub app_id: &'s str,
}

#[derive(Deserialize, Debug)]
pub struct DebugInfo<'s> {
    pub host: &'s str,
    pub started: Option<&'s str>,
    pub build_number: Option<u32>,
    pub approximate_connection_time: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct EventsApiPayload<'s> {
    pub team_id: &'s str,
    #[serde(borrow = "'s")]
    pub event: self::events_api::Event<'s>,
}
