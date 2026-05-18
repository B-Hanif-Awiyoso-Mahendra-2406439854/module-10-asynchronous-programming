use futures_util::{SinkExt, StreamExt};
use std::{error::Error, net::SocketAddr};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast::{channel, Sender},
};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

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
                        let text = message.as_text().unwrap().to_string();
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
