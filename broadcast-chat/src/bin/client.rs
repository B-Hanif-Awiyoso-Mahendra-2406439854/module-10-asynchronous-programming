use futures_util::{SinkExt, StreamExt};
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut websocket, _) = ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
        .connect()
        .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    println!("Connected to ws://127.0.0.1:8080");
    println!("Type a message and press Enter.");

    loop {
        tokio::select! {
            line = stdin.next_line() => {
                match line {
                    Ok(Some(text)) => {
                        if text.trim().is_empty() {
                            continue;
                        }
                        websocket.send(Message::text(text)).await?;
                    }
                    Ok(None) => break,
                    Err(error) => {
                        eprintln!("stdin error: {error}");
                        break;
                    }
                }
            }
            message = websocket.next() => {
                match message {
                    Some(Ok(message)) if message.is_text() => {
                        println!("{}", message.as_text().unwrap());
                    }
                    Some(Ok(message)) if message.is_close() => break,
                    Some(Ok(_)) => {}
                    Some(Err(error)) => return Err(error),
                    None => break,
                }
            }
        }
    }

    Ok(())
}
