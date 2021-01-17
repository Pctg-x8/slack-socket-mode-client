#[async_std::main]
async fn main() {
    env_logger::init();

    let dr = slack_socket_mode_client::run(env!("SLACK_APP_TOKEN"), &mut EventHandler)
        .await
        .expect("Failed to run socket mode client");
    println!("disconnected: {:?}", dr);
}

pub struct EventHandler;
impl slack_socket_mode_client::EventHandler for EventHandler {
    fn on_hello(
        &mut self,
        _: slack_socket_mode_client::protocol::ConnectionInfo,
        _: u32,
        d: slack_socket_mode_client::protocol::DebugInfo,
    ) {
        println!("Hello! approx_connection_time: {}s", d.approximate_connection_time.unwrap_or(0));
    }
}
