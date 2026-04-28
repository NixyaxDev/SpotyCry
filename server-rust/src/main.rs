mod cli;
mod network;
mod songs;

use std::sync::{Arc, Mutex};

use tokio::sync::watch;

#[tokio::main]
async fn main() {
    println!("🚀 Iniciando servidor SpotiCry...");

    let address = "127.0.0.1:8080";
    let song_library = Arc::new(Mutex::new(songs::SongLibrary::new()));
    let (shutdown_sender, shutdown_receiver) = watch::channel(false);

    cli::start_admin_cli(Arc::clone(&song_library), shutdown_sender);

    if let Err(error) =
        network::websocket_server::start_server(address, Arc::clone(&song_library), shutdown_receiver)
            .await
    {
        eprintln!("❌ Error al iniciar el servidor: {}", error);
    }

    println!("👋 Servidor SpotiCry finalizado.");
}
