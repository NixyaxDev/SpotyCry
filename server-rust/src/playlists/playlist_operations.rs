use std::cmp::Ordering;

use crate::playlists::Playlist;
use crate::songs::SongSummary;

#[derive(Debug)]
pub enum PlaylistOperationError {
    SongAlreadyInPlaylist,
    SongNotInPlaylist,
    InvalidFilterCriteria,
    InvalidSortCriteria,
    InvalidSortDirection,
}

#[derive(Clone, Copy)]
pub enum FilterCriteria {
    Title,
    Artist,
    Genre,
}

#[derive(Clone, Copy)]
pub enum SortCriteria {
    Title,
    Artist,
    Duration,
}

#[derive(Clone, Copy)]
pub enum SortDirection {
    Asc,
    Desc,
}

pub fn add_song_to_playlist(
    playlist: &Playlist,
    song_id: &str,
) -> Result<Playlist, PlaylistOperationError> {
    if playlist.song_ids.iter().any(|id| id == song_id) {
        return Err(PlaylistOperationError::SongAlreadyInPlaylist);
    }

    let song_ids = playlist
        .song_ids
        .iter()
        .cloned()
        .chain(std::iter::once(song_id.to_string()))
        .collect();

    Ok(Playlist {
        id: playlist.id.clone(),
        name: playlist.name.clone(),
        song_ids,
    })
}

pub fn remove_song_from_playlist(
    playlist: &Playlist,
    song_id: &str,
) -> Result<Playlist, PlaylistOperationError> {
    if !playlist.song_ids.iter().any(|id| id == song_id) {
        return Err(PlaylistOperationError::SongNotInPlaylist);
    }

    let song_ids = playlist
        .song_ids
        .iter()
        .filter(|id| id.as_str() != song_id)
        .cloned()
        .collect();

    Ok(Playlist {
        id: playlist.id.clone(),
        name: playlist.name.clone(),
        song_ids,
    })
}

pub fn parse_filter_criteria(value: &str) -> Result<FilterCriteria, PlaylistOperationError> {
    match normalize_value(value).as_str() {
        "title" => Ok(FilterCriteria::Title),
        "artist" => Ok(FilterCriteria::Artist),
        "genre" => Ok(FilterCriteria::Genre),
        _ => Err(PlaylistOperationError::InvalidFilterCriteria),
    }
}

pub fn parse_sort_criteria(value: &str) -> Result<SortCriteria, PlaylistOperationError> {
    match normalize_value(value).as_str() {
        "title" => Ok(SortCriteria::Title),
        "artist" => Ok(SortCriteria::Artist),
        "duration" => Ok(SortCriteria::Duration),
        _ => Err(PlaylistOperationError::InvalidSortCriteria),
    }
}

pub fn parse_sort_direction(value: &str) -> Result<SortDirection, PlaylistOperationError> {
    match normalize_value(value).as_str() {
        "asc" => Ok(SortDirection::Asc),
        "desc" => Ok(SortDirection::Desc),
        _ => Err(PlaylistOperationError::InvalidSortDirection),
    }
}

pub fn filter_playlist_songs(
    songs: &[SongSummary],
    criteria: FilterCriteria,
    value: &str,
) -> Vec<SongSummary> {
    let normalized_query = normalize_value(value);

    if normalized_query.is_empty() {
        return songs.to_vec();
    }

    songs
        .iter()
        .filter(|song| match criteria {
            FilterCriteria::Title => normalize_value(&song.title).contains(&normalized_query),
            FilterCriteria::Artist => normalize_optional(&song.artist).contains(&normalized_query),
            FilterCriteria::Genre => normalize_optional(&song.genre).contains(&normalized_query),
        })
        .cloned()
        .collect()
}

pub fn sort_playlist_songs(
    songs: &[SongSummary],
    criteria: SortCriteria,
    direction: SortDirection,
) -> Vec<SongSummary> {
    let mut sorted_songs = songs.to_vec();

    // Rust sorting mutates a temporary local copy only. The original playlist state is unchanged.
    sorted_songs.sort_by(|left, right| {
        let ordering = match criteria {
            SortCriteria::Title => normalize_value(&left.title).cmp(&normalize_value(&right.title)),
            SortCriteria::Artist => {
                normalize_optional(&left.artist).cmp(&normalize_optional(&right.artist))
            }
            SortCriteria::Duration => compare_optional_u64(left.duration, right.duration),
        };

        match direction {
            SortDirection::Asc => ordering,
            SortDirection::Desc => ordering.reverse(),
        }
    });

    sorted_songs
}

fn normalize_value(value: &str) -> String {
    value.trim().to_lowercase()
}

fn normalize_optional(value: &Option<String>) -> String {
    value.as_deref().map(normalize_value).unwrap_or_default()
}

fn compare_optional_u64(left: Option<u64>, right: Option<u64>) -> Ordering {
    match (left, right) {
        (Some(left), Some(right)) => left.cmp(&right),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}
