use serde::Serialize;

use crate::playlists::{Playlist, PlaylistSummary};
use crate::protocol::error::ErrorBody;
use crate::songs::SongSummary;

#[derive(Debug, Serialize)]
pub struct SongDto {
    pub id: String,
    pub title: String,
    pub artist: Option<String>,
    pub genre: Option<String>,
    pub duration: Option<u64>,
}

impl From<SongSummary> for SongDto {
    fn from(summary: SongSummary) -> Self {
        Self {
            id: summary.id,
            title: summary.title,
            artist: summary.artist,
            genre: summary.genre,
            duration: summary.duration,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ListSongsData {
    pub songs: Vec<SongDto>,
}

#[derive(Debug, Serialize)]
pub struct PlaylistDto {
    pub id: String,
    pub name: String,
    pub song_ids: Vec<String>,
}

impl From<Playlist> for PlaylistDto {
    fn from(playlist: Playlist) -> Self {
        Self {
            id: playlist.id,
            name: playlist.name,
            song_ids: playlist.song_ids,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ListPlaylistsData {
    pub playlists: Vec<PlaylistDto>,
}

#[derive(Debug, Serialize)]
pub struct CreatePlaylistData {
    pub playlist: PlaylistDto,
}

#[derive(Debug, Serialize)]
pub struct PlaylistData {
    pub playlist: PlaylistDto,
}

#[derive(Debug, Serialize)]
pub struct PlaylistSummaryDto {
    pub song_count: usize,
    pub total_duration_seconds: u64,
    pub unknown_duration_count: usize,
}

impl From<PlaylistSummary> for PlaylistSummaryDto {
    fn from(summary: PlaylistSummary) -> Self {
        Self {
            song_count: summary.song_count,
            total_duration_seconds: summary.total_duration_seconds,
            unknown_duration_count: summary.unknown_duration_count,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PlaylistSummaryData {
    pub summary: PlaylistSummaryDto,
}

#[derive(Debug, Serialize)]
pub struct StartPlaybackData {
    pub stream_id: String,
    pub song_id: String,
    pub title: String,
    pub mime_type: String,
    pub chunk_size: usize,
}

#[derive(Debug, Serialize)]
pub struct StopPlaybackData {
    pub stream_id: String,
    pub song_id: String,
    pub stopped: bool,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse<T: Serialize> {
    pub request_id: String,
    pub status: &'static str,
    pub data: T,
}

impl<T: Serialize> SuccessResponse<T> {
    pub fn new(request_id: impl Into<String>, data: T) -> Self {
        Self {
            request_id: request_id.into(),
            status: "success",
            data,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub request_id: String,
    pub status: &'static str,
    pub error: ErrorBody,
}

impl ErrorResponse {
    pub fn new(request_id: impl Into<String>, error: ErrorBody) -> Self {
        Self {
            request_id: request_id.into(),
            status: "error",
            error,
        }
    }
}
