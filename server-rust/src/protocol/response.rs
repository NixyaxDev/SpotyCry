use serde::Serialize;

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
