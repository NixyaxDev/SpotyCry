use crate::songs::Song;

pub struct SongLibrary {
    songs: Vec<Song>,
    next_id: usize,
}

impl SongLibrary {
    pub fn new() -> Self {
        Self {
            songs: Vec::new(),
            next_id: 1,
        }
    }

    pub fn songs(&self) -> &[Song] {
        &self.songs
    }

    pub fn active_songs(&self) -> Vec<&Song> {
        self.songs.iter().filter(|song| song.is_active).collect()
    }

    pub fn add_song(&mut self, name: String, path: String) -> Song {
        let song = Song {
            id: format!("song-{:03}", self.next_id),
            name,
            path,
            is_active: false,
        };

        self.next_id += 1;
        self.songs.push(song.clone());
        song
    }

    pub fn delete_song(&mut self, song_id: &str) -> Result<Song, String> {
        let index = self
            .songs
            .iter()
            .position(|song| song.id == song_id)
            .ok_or_else(|| "Song not found".to_string())?;

        Ok(self.songs.remove(index))
    }
}
