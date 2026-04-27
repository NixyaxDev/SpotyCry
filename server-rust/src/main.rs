mod network;

#[tokio::main]
async fn main() {
    println!("🚀 Iniciando servidor SpotiCry...");

    let address = "127.0.0.1:8080";

    if let Err(error) = network::websocket_server::start_server(address).await {
        eprintln!("❌ Error al iniciar el servidor: {}", error);
    }
}