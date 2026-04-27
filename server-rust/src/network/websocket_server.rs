use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

use crate::network::connection_handler::handle_connection;

pub async fn start_server(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(address).await?;

    println!("🌐 Servidor WebSocket escuchando en ws://{}", address);

    loop {
        let (stream, client_address) = listener.accept().await?;

        println!("🔌 Nueva conexión TCP desde: {}", client_address);

        tokio::spawn(async move {
            match accept_async(stream).await {
                Ok(websocket_stream) => {
                    println!("✅ Handshake WebSocket completado con {}", client_address);
                    handle_connection(websocket_stream, client_address.to_string()).await;
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
}