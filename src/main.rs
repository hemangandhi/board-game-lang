extern crate futures_util;
extern crate serde;
extern crate serde_json;
extern crate tokio;
extern crate tokio_tungstenite;

use std::fmt::Display;
use std::sync::Arc;
use std::{env, io::Error};

use futures_util::{stream::SplitSink, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

mod stream_util;

#[derive(Serialize, Deserialize)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize)]
struct ErrorMessage {
    error: String,
}

fn flip_pt<E>(p: Point) -> Result<Point, E> {
    Ok(Point { x: p.y, y: p.x })
}

type OutVecSink = stream_util::VecSink<Arc<SplitSink<WebSocketStream<TcpStream>, Message>>>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    let out_stream: OutVecSink = stream_util::VecSink::new();

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

fn result_to_string<E: Display>(res: Result<String, E>) -> String {
    match res {
        Ok(s) => s,
        Err(e) => serde_json::to_string(&ErrorMessage {
            error: format!("{}", e),
        })
        .unwrap(),
    }
}

async fn accept_connection(stream: TcpStream, ) {
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    // write is a Stream<Output = Result<Message, WsError>>
    // read is the corresponding Sink, but rustc knows the exact types better
    // than I do.
    let (write, read) = ws_stream.split();
    read.filter_map(|r| async move {
        match r {
            Ok(msg) => {
                if msg.is_text() {
                    // Forward the WsError if `into_text` fails. Not sure if
                    // it makes sense.
                    Some(msg.into_text().and_then(|rr| {
                        Ok(Message::text(result_to_string(
                            serde_json::from_str(&rr)
                                .and_then(flip_pt)
                                .and_then(|p| serde_json::to_string(&p)),
                        )))
                    }))
                } else {
                    None
                }
            }
            // It seems like the easiest thing to do is forward the WsErrors.
            // I'm not certain about the semantics.
            Err(e) => Some(Err(e)),
        }
    })
    .forward(write)
    .await
    .expect("Failed to forward message")
}
