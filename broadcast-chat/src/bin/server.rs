use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{error::Error, net::SocketAddr};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast::{channel, Sender},
};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

#[derive(Deserialize, Serialize)]
struct ChatPayload {
    from: String,
    text: String,
}

async fn handle_connection(
    addr: SocketAddr,
    mut websocket: WebSocketStream<TcpStream>,
    broadcast_sender: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut broadcast_receiver = broadcast_sender.subscribe();

    loop {
        tokio::select! {
            incoming = websocket.next() => {
                match incoming {
                    Some(Ok(message)) if message.is_text() => {
                        let text = normalize_message(addr, message.as_text().unwrap());
                        let _ = broadcast_sender.send(text);
                    }
                    Some(Ok(message)) if message.is_close() => break,
                    Some(Ok(_)) => {}
                    Some(Err(error)) => return Err(error.into()),
                    None => break,
                }
            }
            broadcast = broadcast_receiver.recv() => {
                let text = broadcast?;
                websocket.send(Message::text(text)).await?;
            }
        }
    }

    println!("{addr} disconnected");
    Ok(())
}

fn normalize_message(addr: SocketAddr, text: &str) -> String {
    match serde_json::from_str::<ChatPayload>(text) {
        Ok(mut payload) => {
            if payload.from.trim().is_empty() {
                payload.from = addr.to_string();
            }
            serde_json::to_string(&payload)
                .unwrap_or_else(|_| format!(r#"{{"from":"{addr}","text":"{text}"}}"#))
        }
        Err(_) => {
            let payload = ChatPayload {
                from: addr.to_string(),
                text: text.to_string(),
            };
            serde_json::to_string(&payload)
                .unwrap_or_else(|_| format!(r#"{{"from":"{addr}","text":"{text}"}}"#))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (broadcast_sender, _) = channel(16);
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("listening on port 8080");
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let broadcast_sender = broadcast_sender.clone();

        tokio::spawn(async move {
            let (_request, websocket) = ServerBuilder::new().accept(socket).await?;
            handle_connection(addr, websocket, broadcast_sender).await
        });
    }
}
