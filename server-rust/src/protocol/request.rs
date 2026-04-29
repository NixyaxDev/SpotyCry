use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct ClientRequest {
    pub request_id: String,
    pub action: String,
    #[serde(default)]
    pub payload: Value,
}

#[derive(Debug, Deserialize)]
pub struct SearchSongsPayload {
    pub criteria: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct StartPlaybackPayload {
    pub song_id: String,
}

#[derive(Debug, Deserialize)]
pub struct StopPlaybackPayload {
    pub stream_id: String,
    pub song_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePlaylistPayload {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct PlaylistSongPayload {
    pub playlist_id: String,
    pub song_id: String,
}

#[derive(Debug, Deserialize)]
pub struct FilterPlaylistSongsPayload {
    pub playlist_id: String,
    pub criteria: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct SortPlaylistSongsPayload {
    pub playlist_id: String,
    pub criteria: String,
    pub direction: String,
}

#[derive(Debug, Deserialize)]
pub struct PlaylistSummaryPayload {
    pub playlist_id: String,
}
