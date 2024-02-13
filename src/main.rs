mod models;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, Query, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_extra::{headers::UserAgent, TypedHeader};
use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use models::message::{ChatMessage, ChatMessageKind};
use serde::Deserialize;
use std::{net::SocketAddr, ops::ControlFlow};
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct WSParams {
    room_id: String,
    username: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/", ServeDir::new("static"))
        .route("/ws", get(ws_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn ws_handler(
    Query(params): Query<WSParams>,
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<UserAgent>>,
    addr: ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let addr = addr.to_string();
    let room_id = params.room_id.clone();
    let username = params.username.clone();

    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("{username} at {addr} connected in room {room_id} with {user_agent}");

    ws.on_upgrade(move |socket| handle_socket(socket, addr, room_id, username))
}

async fn handle_socket(socket: WebSocket, who: String, room_id: String, username: String) {
    let (mut sender, mut receiver) = socket.split();

    sender
        .send(Message::Text(
            ChatMessage::new(&room_id, &username, "", ChatMessageKind::Join, None).to_string(),
        ))
        .await
        .unwrap();

    let _recv_task = tokio::spawn(async move {
        let mut cnt = 0;
        while let Some(Ok(msg)) = receiver.next().await {
            cnt += 1;
            // print message and break if instructed to do so
            if process_message(&msg, &who, &mut sender).await.is_break() {
                break;
            }
        }

        cnt
    });
}

async fn process_message(
    msg: &Message,
    who: &str,
    sender: &mut SplitSink<WebSocket, Message>,
) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            println!(">>> {who} sent str: {t:?}");
            let message = ChatMessage::parse(t);
            sender
                .send(Message::Text(message.to_string()))
                .await
                .unwrap();
        }
        Message::Binary(d) => {
            println!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who, cf.code, cf.reason
                );
            } else {
                println!(">>> {who} somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            println!(">>> {who} sent pong with {v:?}");
        }
        Message::Ping(v) => {
            println!(">>> {who} sent ping with {v:?}");
        }
    }
    ControlFlow::Continue(())
}
