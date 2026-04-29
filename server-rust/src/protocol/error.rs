use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
}

impl ErrorBody {
    pub fn invalid_json() -> Self {
        Self {
            code: "INVALID_JSON".to_string(),
            message: "Request body is not valid JSON".to_string(),
        }
    }

    pub fn unsupported_action(action: &str) -> Self {
        Self {
            code: "UNSUPPORTED_ACTION".to_string(),
            message: format!("Unsupported action: {}", action),
        }
    }

    pub fn internal_error() -> Self {
        Self {
            code: "INTERNAL_ERROR".to_string(),
            message: "Could not access the song catalog".to_string(),
        }
    }

    pub fn invalid_search_criteria() -> Self {
        Self {
            code: "INVALID_SEARCH_CRITERIA".to_string(),
            message: "Only title search is supported in HU-07".to_string(),
        }
    }

    pub fn invalid_payload() -> Self {
        Self {
            code: "INVALID_PAYLOAD".to_string(),
            message: "Request payload is invalid".to_string(),
        }
    }

    pub fn invalid_playlist_name() -> Self {
        Self {
            code: "INVALID_PLAYLIST_NAME".to_string(),
            message: "Playlist name cannot be empty".to_string(),
        }
    }

    pub fn playlist_already_exists() -> Self {
        Self {
            code: "PLAYLIST_ALREADY_EXISTS".to_string(),
            message: "A playlist with that name already exists".to_string(),
        }
    }

    pub fn playlist_not_found() -> Self {
        Self {
            code: "PLAYLIST_NOT_FOUND".to_string(),
            message: "Playlist not found".to_string(),
        }
    }

    pub fn song_already_in_playlist() -> Self {
        Self {
            code: "SONG_ALREADY_IN_PLAYLIST".to_string(),
            message: "Song is already in the selected playlist".to_string(),
        }
    }

    pub fn song_not_in_playlist() -> Self {
        Self {
            code: "SONG_NOT_IN_PLAYLIST".to_string(),
            message: "Song is not in the selected playlist".to_string(),
        }
    }

    pub fn invalid_filter_criteria() -> Self {
        Self {
            code: "INVALID_FILTER_CRITERIA".to_string(),
            message: "Only title, artist and genre filters are supported".to_string(),
        }
    }

    pub fn invalid_sort_criteria() -> Self {
        Self {
            code: "INVALID_SORT_CRITERIA".to_string(),
            message: "Only title, artist and duration sorting are supported".to_string(),
        }
    }

    pub fn invalid_sort_direction() -> Self {
        Self {
            code: "INVALID_SORT_DIRECTION".to_string(),
            message: "Sort direction must be asc or desc".to_string(),
        }
    }

    pub fn song_not_found() -> Self {
        Self {
            code: "SONG_NOT_FOUND".to_string(),
            message: "Song not found".to_string(),
        }
    }

    pub fn file_not_found() -> Self {
        Self {
            code: "FILE_NOT_FOUND".to_string(),
            message: "Song file was not found on disk".to_string(),
        }
    }

    pub fn stream_error() -> Self {
        Self {
            code: "STREAM_ERROR".to_string(),
            message: "Could not stream the selected song".to_string(),
        }
    }

    pub fn playback_not_found() -> Self {
        Self {
            code: "STREAM_ERROR".to_string(),
            message: "Playback stream not found".to_string(),
        }
    }
}
