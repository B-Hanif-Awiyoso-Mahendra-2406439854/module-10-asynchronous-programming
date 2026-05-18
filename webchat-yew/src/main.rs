use futures_util::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

const WEBSOCKET_URL: &str = "ws://127.0.0.1:8080";

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct ChatPayload {
    #[serde(default)]
    from: String,
    text: String,
}

#[function_component(App)]
fn app() -> Html {
    let messages = use_state(|| {
        vec![ChatPayload {
            from: "system".to_string(),
            text: "Welcome to the Rust-powered YewChat room.".to_string(),
        }]
    });
    let draft = use_state(String::new);
    let writer = use_mut_ref(|| None::<futures_util::stream::SplitSink<WebSocket, Message>>);
    let connected = use_state(|| false);

    {
        let messages = messages.clone();
        let writer = writer.clone();
        let connected = connected.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                match WebSocket::open(WEBSOCKET_URL) {
                    Ok(socket) => {
                        connected.set(true);
                        let (write, mut read) = socket.split();
                        *writer.borrow_mut() = Some(write);

                        while let Some(incoming) = read.next().await {
                            match incoming {
                                Ok(Message::Text(text)) => {
                                    let payload = serde_json::from_str::<ChatPayload>(&text)
                                        .unwrap_or_else(|_| ChatPayload {
                                            from: "server".to_string(),
                                            text,
                                        });
                                    let mut next = (*messages).clone();
                                    next.push(payload);
                                    messages.set(next);
                                }
                                Ok(Message::Bytes(_)) => {}
                                Err(_) => {
                                    let mut next = (*messages).clone();
                                    next.push(ChatPayload {
                                        from: "system".to_string(),
                                        text: "Connection closed. Restart the server, then refresh this page.".to_string(),
                                    });
                                    messages.set(next);
                                    connected.set(false);
                                    break;
                                }
                            }
                        }
                    }
                    Err(_) => {
                        let mut next = (*messages).clone();
                        next.push(ChatPayload {
                            from: "system".to_string(),
                            text: format!("Could not connect to {WEBSOCKET_URL}. Start the websocket server first."),
                        });
                        messages.set(next);
                    }
                }
            });

            || ()
        });
    }

    let oninput = {
        let draft = draft.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            draft.set(input.value());
        })
    };

    let send_message = {
        let draft = draft.clone();
        let writer = writer.clone();
        Callback::from(move |_| {
            let text = (*draft).trim().to_string();
            if text.is_empty() {
                return;
            }
            draft.set(String::new());

            let writer = writer.clone();
            spawn_local(async move {
                let payload = ChatPayload {
                    from: "browser-client".to_string(),
                    text,
                };
                if let Ok(serialized) = serde_json::to_string(&payload) {
                    if let Some(writer) = writer.borrow_mut().as_mut() {
                        let _ = writer.send(Message::Text(serialized)).await;
                    }
                }
            });
        })
    };

    let onkeypress = {
        let send_message = send_message.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "Enter" {
                send_message.emit(());
            }
        })
    };

    let onclick = {
        let send_message = send_message.clone();
        Callback::from(move |_| send_message.emit(()))
    };

    html! {
        <div class="app">
            <aside class="sidebar">
                <div class="brand">
                    <h1>{"YewChat"}</h1>
                    <p>{"A browser chat client connected to the Rust websocket server from Tutorial 2."}</p>
                </div>
                <div class={classes!("status", if *connected { "online" } else { "offline" })}>
                    { if *connected { "Connected" } else { "Waiting for server" } }
                </div>
                <p class="tip">{"Creative touch: responsive layout, sender labels, and JSON messages for browser clients."}</p>
            </aside>

            <section class="chat">
                <div class="toolbar">
                    <h2>{"Module 10 WebChat"}</h2>
                    <span>{WEBSOCKET_URL}</span>
                </div>
                <div class="messages">
                    { for messages.iter().map(|message| html! {
                        <article class="message">
                            <strong>{ &message.from }</strong>
                            <span>{ &message.text }</span>
                        </article>
                    })}
                </div>
                <div class="composer">
                    <input
                        value={(*draft).clone()}
                        {oninput}
                        {onkeypress}
                        placeholder="Write a message"
                    />
                    <button {onclick} disabled={!*connected}>{"Send"}</button>
                </div>
            </section>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
