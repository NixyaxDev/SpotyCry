use std::sync::{Arc, Mutex};

use tokio::net::TcpListener;
use tokio::sync::watch;
use tokio_tungstenite::accept_async;

use crate::network::connection_handler::handle_connection;
use crate::songs::SongLibrary;

pub async fn start_server(
    address: &str,
    song_library: Arc<Mutex<SongLibrary>>,
    mut shutdown_receiver: watch::Receiver<bool>,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(address).await?;

    println!("🌐 Servidor WebSocket escuchando en ws://{}", address);

    loop {
        tokio::select! {
            accept_result = listener.accept() => {
                let (stream, client_address) = accept_result?;

                println!("🔌 Nueva conexión TCP desde: {}", client_address);
                let song_library = Arc::clone(&song_library);

                tokio::spawn(async move {
                    match accept_async(stream).await {
                        Ok(websocket_stream) => {
                            println!("✅ Handshake WebSocket completado con {}", client_address);
                            handle_connection(
                                websocket_stream,
                                client_address.to_string(),
                                song_library,
                            )
                            .await;
                        }
                        Err(error) => {
                            eprintln!(
                                "❌ Error durante el handshake WebSocket con {}: {}",
                                client_address, error
                            );
                        }
                    }
                });
            }
            changed = shutdown_receiver.changed() => {
                match changed {
                    Ok(_) if *shutdown_receiver.borrow() => {
                        println!("🛑 Shutdown signal received. Stopping WebSocket server.");
                        break;
                    }
                    Ok(_) => {}
                    Err(_) => {
                        println!("🛑 Shutdown channel closed. Stopping WebSocket server.");
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}
