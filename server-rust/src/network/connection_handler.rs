use std::sync::{Arc, Mutex};

use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

use crate::protocol::error::ErrorBody;
use crate::protocol::request::ClientRequest;
use crate::protocol::response::{ErrorResponse, ListSongsData, SongDto, SuccessResponse};
use crate::songs::SongLibrary;

pub async fn handle_connection(
    mut websocket_stream: WebSocketStream<TcpStream>,
    client_address: String,
    song_library: Arc<Mutex<SongLibrary>>,
) {
    println!("👤 Cliente conectado: {}", client_address);

    while let Some(message_result) = websocket_stream.next().await {
        match message_result {
            Ok(Message::Text(text)) => {
                println!("📩 Mensaje recibido de {}: {}", client_address, text);

                let response = handle_text_message(&text, &song_library);

                if let Err(error) = websocket_stream.send(Message::Text(response)).await {
                    eprintln!(
                        "❌ Error enviando respuesta a {}: {}",
                        client_address, error
                    );
                    break;
                }
            }

            Ok(Message::Binary(bytes)) => {
                println!(
                    "📦 Mensaje binario recibido de {}: {} bytes",
                    client_address,
                    bytes.len()
                );
            }

            Ok(Message::Close(_)) => {
                println!("🔴 Cliente desconectado: {}", client_address);
                break;
            }

            Ok(Message::Ping(payload)) => {
                println!("🏓 Ping recibido de {}", client_address);

                if let Err(error) = websocket_stream.send(Message::Pong(payload)).await {
                    eprintln!(
                        "❌ Error enviando Pong a {}: {}",
                        client_address, error
                    );
                    break;
                }
            }

            Ok(Message::Pong(_)) => {
                println!("🏓 Pong recibido de {}", client_address);
            }

            Err(error) => {
                eprintln!(
                    "❌ Error en la conexión con {}: {}",
                    client_address, error
                );
                break;
            }

            _ => {}
        }
    }

    println!("🧹 Conexión finalizada: {}", client_address);
}

fn handle_text_message(text: &str, song_library: &Arc<Mutex<SongLibrary>>) -> String {
    match serde_json::from_str::<ClientRequest>(text) {
        Ok(request) => handle_request(request, song_library),
        Err(_) => serialize_response(&ErrorResponse::new("unknown", ErrorBody::invalid_json())),
    }
}

fn handle_request(request: ClientRequest, song_library: &Arc<Mutex<SongLibrary>>) -> String {
    match request.action.as_str() {
        "list_songs" => {
            let _ = &request.payload;

            match song_library.lock() {
                Ok(library) => {
                    let songs = library
                        .song_summaries()
                        .into_iter()
                        .map(SongDto::from)
                        .collect();

                    serialize_response(&SuccessResponse::new(
                        request.request_id,
                        ListSongsData { songs },
                    ))
                }
                Err(_) => serialize_response(&ErrorResponse::new(
                    request.request_id,
                    ErrorBody::internal_error(),
                )),
            }
        }
        _ => serialize_response(&ErrorResponse::new(
            request.request_id,
            ErrorBody::unsupported_action(&request.action),
        )),
    }
}

fn serialize_response<T: serde::Serialize>(response: &T) -> String {
    serde_json::to_string(response)
        .unwrap_or_else(|_| "{\"request_id\":\"unknown\",\"status\":\"error\",\"error\":{\"code\":\"INTERNAL_ERROR\",\"message\":\"Could not serialize response\"}}".to_string())
}
