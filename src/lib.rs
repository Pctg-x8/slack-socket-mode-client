//! Slack Socket Mode API Client Library

use futures_util::{SinkExt, StreamExt};
use url::Url;

pub mod protocol;

#[allow(unused_variables)]
pub trait EventHandler {
    fn on_hello(
        &mut self,
        connection_info: protocol::ConnectionInfo,
        num_connections: u32,
        debug_info: protocol::DebugInfo,
    ) {
    }

    fn on_events_api(&mut self, payload: protocol::EventsApiPayload) {}
}

#[derive(Debug, Clone)]
pub enum DisconnectReason {
    RefreshRequested,
    Other(String),
    Unknown,
}

#[derive(Debug)]
pub enum RunError {
    HttpClientError(HttpClientError),
    OpenConnectionApiError(Option<String>),
    UrlParseError(url::ParseError),
    #[cfg(feature = "runtime-async-std")]
    TcpStreamConnectionError(async_std::io::Error),
    TlsConnectionError(std::io::Error),
    WebSocketError(tungstenite::Error),
}
impl From<HttpClientError> for RunError {
    fn from(e: HttpClientError) -> Self {
        Self::HttpClientError(e)
    }
}
impl From<url::ParseError> for RunError {
    fn from(e: url::ParseError) -> Self {
        Self::UrlParseError(e)
    }
}
impl From<tungstenite::Error> for RunError {
    fn from(e: tungstenite::Error) -> Self {
        Self::WebSocketError(e)
    }
}

pub async fn run<H: EventHandler + ?Sized>(token: &str, handler: &mut H) -> Result<DisconnectReason, RunError> {
    let ws_url = open_connection(token)
        .await?
        .map_err(RunError::OpenConnectionApiError)?;
    let ws_parsed = Url::parse(&ws_url)?;
    let ws_domain = ws_parsed.domain().expect("WebSocket URL doesn't have domain");

    #[cfg(feature = "runtime-async-std")]
    let tcp_stream = async_std::net::TcpStream::connect((ws_domain, 443))
        .await
        .map_err(RunError::TcpStreamConnectionError)?;
    let enc_stream = async_tls::TlsConnector::default()
        .connect(ws_domain, tcp_stream)
        .await
        .map_err(RunError::TlsConnectionError)?;
    let (mut ws, _) = async_tungstenite::client_async(&ws_url, enc_stream).await?;

    while let Some(msg) = ws.next().await {
        match msg? {
            tungstenite::Message::Text(t) => match serde_json::from_str(&t) {
                Ok(protocol::Message::Hello {
                    num_connections,
                    connection_info,
                    debug_info,
                }) => {
                    handler.on_hello(connection_info, num_connections, debug_info);
                }
                Ok(protocol::Message::Disconnect { reason, .. }) => {
                    return match reason {
                        "refresh_requested" => Ok(DisconnectReason::RefreshRequested),
                        s => Ok(DisconnectReason::Other(String::from(s))),
                    }
                }
                Ok(protocol::Message::EventsApi { envelope_id, payload }) => {
                    ws.send(tungstenite::Message::text(
                        serde_json::to_string(&protocol::Acknowledge {
                            envelope_id: &envelope_id,
                            payload: None,
                        })
                        .expect("Failed to serialize ack"),
                    ))
                    .await?;

                    handler.on_events_api(payload);
                }
                Err(e) => {
                    log::error!("Failed to parse incoming message: {}: {:?}", t, e);
                }
            },
            tungstenite::Message::Ping(p) => {
                ws.send(tungstenite::Message::Pong(p)).await?;
            }
            tungstenite::Message::Close(_) => {
                break;
            }
            m => {
                log::warn!("Unsupported WebSocket Message: {:?}", m);
            }
        }
    }

    Ok(DisconnectReason::Unknown)
}

#[cfg(feature = "runtime-async-std")]
type HttpClientError = surf::Error;

async fn open_connection(token: &str) -> Result<Result<String, Option<String>>, HttpClientError> {
    #[derive(serde::Deserialize)]
    pub struct ApiResponse {
        ok: bool,
        url: Option<String>,
        error: Option<String>,
    }
    let mut tok_bearer = String::with_capacity(token.len() + 7);
    tok_bearer.push_str("Bearer ");
    tok_bearer.push_str(token);

    #[cfg(feature = "runtime-async-std")]
    let r: ApiResponse = surf::post("https://slack.com/api/apps.connections.open")
        .header(surf::http::headers::AUTHORIZATION, tok_bearer)
        .recv_json()
        .await?;

    Ok(if r.ok {
        Ok(r.url.expect("no url returned from api?"))
    } else {
        Err(r.error)
    })
}
