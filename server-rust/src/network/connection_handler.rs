use std::sync::{Arc, Mutex};

use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

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

                let song_count = match song_library.lock() {
                    Ok(library) => library.songs().len(),
                    Err(_) => {
                        eprintln!("❌ Error accediendo a la biblioteca de canciones.");
                        0
                    }
                };

                let response = format!(
                    "Servidor recibió: {} | canciones cargadas: {}",
                    text, song_count
                );

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
