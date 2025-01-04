use bevy::{log::LogPlugin, prelude::*, utils::Duration};
use bevy_mod_websocket::*;
use bevy_tokio_tasks::TokioTasksPlugin;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

const ECHO_URL: &str = "wss://echo.websocket.org/";
const RETRY_INTERVAL: Duration = Duration::from_secs(10);

fn on_success(trigger: Trigger<WebSocketConnectSuccessEvent>) {
    let evt = trigger.event();
    info!("websocket connect success: {:?}", evt);
}

fn on_error(trigger: Trigger<WebSocketErrorEvent>, mut ws_client: WebSocketClient) {
    let evt = trigger.event();
    warn!("websocket error: {:?}", evt.error);

    ws_client.retry(trigger.entity(), evt.request.clone(), RETRY_INTERVAL);
}

fn on_disconnect(trigger: Trigger<WebSocketDisconnectEvent>, mut ws_client: WebSocketClient) {
    let evt = trigger.event();
    warn!("websocket disconnect");

    ws_client.retry(trigger.entity(), evt.request.clone(), RETRY_INTERVAL);
}

fn on_message(trigger: Trigger<WebSocketMessageEvent>) {
    let evt = trigger.event();

    match &evt.message {
        Message::Text(value) => {
            info!("received text message from {}: {}", evt.uri, value);
        }
        Message::Binary(value) => {
            info!("received binary message from {}: {:?}", evt.uri, value);
        }
        _ => {
            warn!("unexpected message from {}: {:?}", evt.uri, evt.message);
        }
    }
}

fn connect(mut client: WebSocketClient) {
    client
        .connect(ECHO_URL.into_client_request().unwrap())
        .on_success(on_success)
        .on_error(on_error)
        .on_disconnect(on_disconnect)
        .on_message(on_message);
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(LogPlugin::default())
        .add_plugins((TokioTasksPlugin::default(), WebSocketPlugin::default()))
        .add_systems(Startup, connect)
        .run();
}
