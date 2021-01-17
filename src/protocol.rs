//! JSON Types

use serde::Deserialize;

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
