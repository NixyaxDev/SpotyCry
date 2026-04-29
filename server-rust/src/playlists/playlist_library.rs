use crate::playlists::{
    add_song_to_playlist, remove_song_from_playlist, Playlist, PlaylistOperationError,
};

#[derive(Debug)]
pub enum PlaylistLibraryError {
    InvalidName,
    AlreadyExists,
    PlaylistNotFound,
    SongAlreadyInPlaylist,
    SongNotInPlaylist,
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

    pub fn find_playlist(&self, playlist_id: &str) -> Option<Playlist> {
        self.playlists
            .iter()
            .find(|playlist| playlist.id == playlist_id)
            .cloned()
    }

    pub fn add_song_to_playlist(
        &mut self,
        playlist_id: &str,
        song_id: &str,
    ) -> Result<Playlist, PlaylistLibraryError> {
        let current_playlist = self
            .find_playlist(playlist_id)
            .ok_or(PlaylistLibraryError::PlaylistNotFound)?;

        let updated_playlist =
            add_song_to_playlist(&current_playlist, song_id).map_err(map_operation_error)?;

        self.replace_playlist(updated_playlist)
    }

    pub fn remove_song_from_playlist(
        &mut self,
        playlist_id: &str,
        song_id: &str,
    ) -> Result<Playlist, PlaylistLibraryError> {
        let current_playlist = self
            .find_playlist(playlist_id)
            .ok_or(PlaylistLibraryError::PlaylistNotFound)?;

        let updated_playlist =
            remove_song_from_playlist(&current_playlist, song_id).map_err(map_operation_error)?;

        self.replace_playlist(updated_playlist)
    }

    fn replace_playlist(
        &mut self,
        updated_playlist: Playlist,
    ) -> Result<Playlist, PlaylistLibraryError> {
        let playlist_index = self
            .playlists
            .iter()
            .position(|playlist| playlist.id == updated_playlist.id)
            .ok_or(PlaylistLibraryError::PlaylistNotFound)?;

        self.playlists[playlist_index] = updated_playlist.clone();
        Ok(updated_playlist)
    }
}

fn normalize_playlist_name(name: &str) -> String {
    name.trim().to_string()
}

fn map_operation_error(error: PlaylistOperationError) -> PlaylistLibraryError {
    match error {
        PlaylistOperationError::SongAlreadyInPlaylist => {
            PlaylistLibraryError::SongAlreadyInPlaylist
        }
        PlaylistOperationError::SongNotInPlaylist => PlaylistLibraryError::SongNotInPlaylist,
        PlaylistOperationError::InvalidFilterCriteria
        | PlaylistOperationError::InvalidSortCriteria
        | PlaylistOperationError::InvalidSortDirection => PlaylistLibraryError::PlaylistNotFound,
    }
}
