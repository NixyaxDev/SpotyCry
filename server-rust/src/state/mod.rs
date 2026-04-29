use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::playback::active_streams::ActiveStreams;
use crate::playlists::PlaylistLibrary;
use crate::songs::SongLibrary;

/// Estado compartido de toda la aplicación del servidor.
///
/// Mantener los recursos compartidos en un solo lugar simplifica pasar el mismo
/// estado al CLI, a los handlers WebSocket y a futuros módulos sin crecer
/// listas largas de parámetros en cada función.
#[derive(Clone)]
pub struct AppState {
    pub songs: Arc<Mutex<SongLibrary>>,
    pub playlists: Arc<Mutex<PlaylistLibrary>>,
    pub active_streams: ActiveStreams,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            songs: Arc::new(Mutex::new(SongLibrary::new())),
            playlists: Arc::new(Mutex::new(PlaylistLibrary::new())),
            active_streams: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
