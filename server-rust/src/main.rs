mod cli;
mod network;
mod playback;
mod playlists;
mod protocol;
mod songs;
mod state;

use tokio::sync::watch;

#[tokio::main]
async fn main() {
    println!("🚀 Iniciando servidor SpotiCry...");

    let address = "127.0.0.1:8080";
    let app_state = state::AppState::new();
    let (shutdown_sender, shutdown_receiver) = watch::channel(false);

    cli::start_admin_cli(app_state.songs.clone(), shutdown_sender);

    if let Err(error) =
        network::websocket_server::start_server(address, app_state, shutdown_receiver).await
    {
        eprintln!("❌ Error al iniciar el servidor: {}", error);
    }

    println!("👋 Servidor SpotiCry finalizado.");
}
