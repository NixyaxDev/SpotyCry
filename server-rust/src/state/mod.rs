use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::playback::active_streams::ActiveStreams;
use crate::playlists::PlaylistLibrary;
use crate::songs::SongLibrary;

/// Shared application state for the whole server process.
///
/// Keeping the shared resources in one place makes it easier to pass the same
/// state to the CLI, WebSocket handlers and future modules without growing
/// long parameter lists everywhere.
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
