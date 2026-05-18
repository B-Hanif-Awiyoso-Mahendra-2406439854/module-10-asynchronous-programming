use futures_util::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

const WEBSOCKET_URL: &str = "ws://127.0.0.1:8080";

#[function_component(App)]
fn app() -> Html {
    let messages = use_state(Vec::<String>::new);
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
                                    let mut next = (*messages).clone();
                                    next.push(text);
                                    messages.set(next);
                                }
                                Ok(Message::Bytes(_)) => {}
                                Err(_) => {
                                    let mut next = (*messages).clone();
                                    next.push(
                                        "Connection closed. Restart the server, then refresh this page."
                                            .to_string(),
                                    );
                                    messages.set(next);
                                    connected.set(false);
                                    break;
                                }
                            }
                        }
                    }
                    Err(_) => {
                        let mut next = (*messages).clone();
                        next.push(format!(
                            "Could not connect to {WEBSOCKET_URL}. Start the websocket server first."
                        ));
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
                if let Some(writer) = writer.borrow_mut().as_mut() {
                    let _ = writer.send(Message::Text(text)).await;
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
            <h1>{"YewChat"}</h1>
            <p class="status">{ if *connected { "Connected" } else { "Waiting for server" } }</p>
            <div class="messages">
                { for messages.iter().map(|message| html! {
                    <p class="message">{ message }</p>
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
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
