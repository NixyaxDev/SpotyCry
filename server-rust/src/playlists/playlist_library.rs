use crate::playlists::Playlist;

#[derive(Debug)]
pub enum PlaylistLibraryError {
    InvalidName,
    AlreadyExists,
}

pub struct PlaylistLibrary {
    playlists: Vec<Playlist>,
    next_id: usize,
}

impl PlaylistLibrary {
    pub fn new() -> Self {
        Self {
            playlists: Vec::new(),
            next_id: 1,
        }
    }

    pub fn playlists(&self) -> Vec<Playlist> {
        self.playlists.iter().cloned().collect()
    }

    pub fn create_playlist(&mut self, name: &str) -> Result<Playlist, PlaylistLibraryError> {
        let normalized_name = normalize_playlist_name(name);

        if normalized_name.is_empty() {
            return Err(PlaylistLibraryError::InvalidName);
        }

        if self.playlists.iter().any(|playlist| {
            normalize_playlist_name(&playlist.name).to_lowercase() == normalized_name.to_lowercase()
        }) {
            return Err(PlaylistLibraryError::AlreadyExists);
        }

        let playlist = Playlist {
            id: format!("playlist-{:03}", self.next_id),
            name: normalized_name,
            song_ids: Vec::new(),
        };

        self.next_id += 1;
        self.playlists.push(playlist.clone());

        Ok(playlist)
    }
}

fn normalize_playlist_name(name: &str) -> String {
    name.trim().to_string()
}
