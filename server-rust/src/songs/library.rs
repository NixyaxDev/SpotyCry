use std::fs;
use std::path::{Path, PathBuf};

use lofty::prelude::{Accessor, AudioFile, TaggedFileExt};
use lofty::probe::Probe;

use crate::songs::Song;

#[derive(Clone, Debug)]
pub struct SongSummary {
    pub id: String,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub duration: Option<u64>,
}

#[derive(Debug)]
pub enum SongLibraryError {
    SongNotFound,
    SongInPlayback,
}

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

    pub fn song_summaries(&self) -> Vec<SongSummary> {
        self.songs
            .iter()
            .map(|song| SongSummary {
                id: song.id.clone(),
                title: song.title.clone(),
                artist: song.artist.clone(),
                album: song.album.clone(),
                genre: song.genre.clone(),
                duration: song.duration,
            })
            .collect()
    }

    pub fn search_songs(&self, criteria: &str, value: &str) -> Option<Vec<SongSummary>> {
        let normalized_query = normalize_search_value(value);

        if normalized_query.is_empty() {
            return Some(self.song_summaries());
        }

        let normalized_criteria = normalize_search_value(criteria);

        let songs = self
            .songs
            .iter()
            .filter(|song| matches_search(song, &normalized_criteria, &normalized_query))
            .map(|song| SongSummary {
                id: song.id.clone(),
                title: song.title.clone(),
                artist: song.artist.clone(),
                album: song.album.clone(),
                genre: song.genre.clone(),
                duration: song.duration,
            })
            .collect();

        match normalized_criteria.as_str() {
            "title" | "artist" | "album" | "genre" => Some(songs),
            _ => None,
        }
    }

    pub fn find_song(&self, song_id: &str) -> Option<Song> {
        self.songs.iter().find(|song| song.id == song_id).cloned()
    }

    pub fn has_song(&self, song_id: &str) -> bool {
        self.songs.iter().any(|song| song.id == song_id)
    }

    pub fn song_summaries_by_ids(&self, song_ids: &[String]) -> Vec<SongSummary> {
        song_ids
            .iter()
            .filter_map(|song_id| {
                self.songs
                    .iter()
                    .find(|song| song.id == *song_id)
                    .map(|song| SongSummary {
                        id: song.id.clone(),
                        title: song.title.clone(),
                        artist: song.artist.clone(),
                        album: song.album.clone(),
                        genre: song.genre.clone(),
                        duration: song.duration,
                    })
            })
            .collect()
    }

    pub fn active_songs(&self) -> Vec<&Song> {
        self.songs.iter().filter(|song| song.is_active).collect()
    }

    pub fn add_song(&mut self, path: &str) -> Result<Song, String> {
        let normalized_path = normalize_file_path(path)?;
        validate_audio_file(&normalized_path)?;

        let file_size = fs::metadata(&normalized_path)
            .map_err(|_| "Could not read file metadata".to_string())?
            .len();
        let metadata = extract_song_metadata(&normalized_path)?;

        if self.is_duplicate(&normalized_path, &metadata.title, file_size) {
            return Err("Song already exists".to_string());
        }

        let song = Song {
            id: format!("song-{:03}", self.next_id),
            title: metadata.title,
            artist: metadata.artist,
            album: metadata.album,
            genre: metadata.genre,
            duration: metadata.duration,
            file_path: normalized_path.to_string_lossy().to_string(),
            is_active: false,
        };

        self.next_id += 1;
        self.songs.push(song.clone());
        Ok(song)
    }

    pub fn delete_song(&mut self, song_id: &str) -> Result<Song, SongLibraryError> {
        if self.is_song_active(song_id) {
            return Err(SongLibraryError::SongInPlayback);
        }

        let index = self
            .songs
            .iter()
            .position(|song| song.id == song_id)
            .ok_or(SongLibraryError::SongNotFound)?;

        Ok(self.songs.remove(index))
    }

    pub fn set_active_song(&mut self, song_id: &str) -> Result<Song, SongLibraryError> {
        let target_index = self
            .songs
            .iter()
            .position(|song| song.id == song_id)
            .ok_or(SongLibraryError::SongNotFound)?;

        for song in &mut self.songs {
            song.is_active = false;
        }

        self.songs[target_index].is_active = true;
        Ok(self.songs[target_index].clone())
    }

    pub fn is_song_active(&self, song_id: &str) -> bool {
        self.songs
            .iter()
            .find(|song| song.id == song_id)
            .map(|song| song.is_active)
            .unwrap_or(false)
    }

    pub fn clear_active_song(&mut self, song_id: &str) {
        if let Some(song) = self.songs.iter_mut().find(|song| song.id == song_id) {
            song.is_active = false;
        }
    }

    fn is_duplicate(&self, normalized_path: &Path, title: &str, file_size: u64) -> bool {
        let normalized_path_str = normalized_path.to_string_lossy();

        self.songs.iter().any(|song| {
            song.file_path == normalized_path_str
                || (song.title == title && song_file_size(song) == Some(file_size))
        })
    }
}

struct SongMetadata {
    title: String,
    artist: Option<String>,
    album: Option<String>,
    genre: Option<String>,
    duration: Option<u64>,
}

fn normalize_file_path(path: &str) -> Result<PathBuf, String> {
    let file_path = Path::new(path);

    if !file_path.exists() {
        return Err("File does not exist".to_string());
    }

    if !file_path.is_file() {
        return Err("Path is not a file".to_string());
    }

    fs::canonicalize(file_path).map_err(|_| "Could not normalize file path".to_string())
}

fn validate_audio_file(path: &Path) -> Result<(), String> {
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .ok_or_else(|| "Unsupported file type".to_string())?;

    match extension.as_str() {
        "mp3" | "wav" => Ok(()),
        _ => Err("Unsupported file type".to_string()),
    }
}

fn matches_search(song: &Song, criteria: &str, normalized_query: &str) -> bool {
    match criteria {
        "title" => title_matches(&song.title, normalized_query),
        "artist" => artist_matches(song.artist.as_deref(), normalized_query),
        "album" => album_matches(song.album.as_deref(), normalized_query),
        "genre" => genre_matches(song.genre.as_deref(), normalized_query),
        _ => false,
    }
}

fn title_matches(title: &str, normalized_query: &str) -> bool {
    // Title search supports partial substring matching anywhere in the song title.
    normalize_search_value(title).contains(normalized_query)
}

fn artist_matches(artist: Option<&str>, normalized_query: &str) -> bool {
    // Artist search matches if any normalized word starts with the query.
    normalize_optional_search_value(artist)
        .split_whitespace()
        .any(|word| word.starts_with(normalized_query))
}

fn album_matches(album: Option<&str>, normalized_query: &str) -> bool {
    // Album search matches the beginning of the normalized album name.
    normalize_optional_search_value(album).starts_with(normalized_query)
}

fn genre_matches(genre: Option<&str>, normalized_query: &str) -> bool {
    // Genre search uses exact normalized equality to make it technically distinct.
    normalize_optional_search_value(genre) == normalized_query
}

fn extract_song_metadata(path: &Path) -> Result<SongMetadata, String> {
    let fallback = fallback_song_metadata(path)?;

    // Metadata extraction should improve the catalog when tags exist, but it
    // should never block song registration. If parsing fails, we keep the
    // filename-based fallback values.
    let tagged_file = match Probe::open(path).and_then(|probe| probe.read()) {
        Ok(tagged_file) => tagged_file,
        Err(_) => return Ok(fallback),
    };

    let properties = tagged_file.properties();
    let primary_tag = tagged_file.primary_tag().or_else(|| tagged_file.first_tag());

    Ok(SongMetadata {
        title: primary_tag
            .and_then(|tag| tag.title().map(|value| value.into_owned()))
            .filter(|title| !title.trim().is_empty())
            .unwrap_or(fallback.title),
        artist: primary_tag
            .and_then(|tag| tag.artist().map(|value| value.into_owned()))
            .filter(|artist| !artist.trim().is_empty()),
        album: primary_tag
            .and_then(|tag| tag.album().map(|value| value.into_owned()))
            .filter(|album| !album.trim().is_empty()),
        genre: primary_tag
            .and_then(|tag| tag.genre().map(|value| value.into_owned()))
            .filter(|genre| !genre.trim().is_empty()),
        duration: Some(properties.duration().as_secs()).filter(|duration| *duration > 0),
    })
}

fn fallback_song_metadata(path: &Path) -> Result<SongMetadata, String> {
    let title = path
        .file_stem()
        .and_then(|name| name.to_str())
        .or_else(|| path.file_name().and_then(|name| name.to_str()))
        .ok_or_else(|| "Invalid file name".to_string())?
        .to_string();

    Ok(SongMetadata {
        title,
        artist: None,
        album: None,
        genre: None,
        duration: None,
    })
}

fn song_file_size(song: &Song) -> Option<u64> {
    fs::metadata(&song.file_path)
        .ok()
        .map(|metadata| metadata.len())
}

fn normalize_search_value(value: &str) -> String {
    value.trim().to_lowercase()
}

fn normalize_optional_search_value(value: Option<&str>) -> String {
    value.map(normalize_search_value).unwrap_or_default()
}
