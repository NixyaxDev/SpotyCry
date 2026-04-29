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
            message: "El cuerpo de la solicitud no contiene un JSON válido".to_string(),
        }
    }

    pub fn unsupported_action(action: &str) -> Self {
        Self {
            code: "UNSUPPORTED_ACTION".to_string(),
            message: format!("Acción no soportada: {}", action),
        }
    }

    pub fn internal_error() -> Self {
        Self {
            code: "INTERNAL_ERROR".to_string(),
            message: "No se pudo acceder al catálogo de canciones".to_string(),
        }
    }

    pub fn invalid_search_criteria() -> Self {
        Self {
            code: "INVALID_SEARCH_CRITERIA".to_string(),
            message: "Solo se soportan búsquedas por título, artista, álbum y género".to_string(),
        }
    }

    pub fn invalid_payload() -> Self {
        Self {
            code: "INVALID_PAYLOAD".to_string(),
            message: "El payload de la solicitud es inválido".to_string(),
        }
    }

    pub fn invalid_playlist_name() -> Self {
        Self {
            code: "INVALID_PLAYLIST_NAME".to_string(),
            message: "El nombre de la playlist no puede estar vacío".to_string(),
        }
    }

    pub fn playlist_already_exists() -> Self {
        Self {
            code: "PLAYLIST_ALREADY_EXISTS".to_string(),
            message: "Ya existe una playlist con ese nombre".to_string(),
        }
    }

    pub fn playlist_not_found() -> Self {
        Self {
            code: "PLAYLIST_NOT_FOUND".to_string(),
            message: "Playlist no encontrada".to_string(),
        }
    }

    pub fn song_already_in_playlist() -> Self {
        Self {
            code: "SONG_ALREADY_IN_PLAYLIST".to_string(),
            message: "La canción ya está en la playlist seleccionada".to_string(),
        }
    }

    pub fn song_not_in_playlist() -> Self {
        Self {
            code: "SONG_NOT_IN_PLAYLIST".to_string(),
            message: "La canción no pertenece a la playlist seleccionada".to_string(),
        }
    }

    pub fn invalid_filter_criteria() -> Self {
        Self {
            code: "INVALID_FILTER_CRITERIA".to_string(),
            message: "Solo se soportan filtros por título, artista y género".to_string(),
        }
    }

    pub fn invalid_sort_criteria() -> Self {
        Self {
            code: "INVALID_SORT_CRITERIA".to_string(),
            message: "Solo se soporta ordenar por título, artista y duración".to_string(),
        }
    }

    pub fn invalid_sort_direction() -> Self {
        Self {
            code: "INVALID_SORT_DIRECTION".to_string(),
            message: "La dirección de ordenamiento debe ser asc o desc".to_string(),
        }
    }

    pub fn song_not_found() -> Self {
        Self {
            code: "SONG_NOT_FOUND".to_string(),
            message: "Canción no encontrada".to_string(),
        }
    }

    pub fn file_not_found() -> Self {
        Self {
            code: "FILE_NOT_FOUND".to_string(),
            message: "El archivo de la canción no se encontró en disco".to_string(),
        }
    }

    pub fn stream_error() -> Self {
        Self {
            code: "STREAM_ERROR".to_string(),
            message: "No se pudo transmitir la canción seleccionada".to_string(),
        }
    }

    pub fn playback_not_found() -> Self {
        Self {
            code: "STREAM_ERROR".to_string(),
            message: "No se encontró el flujo de reproducción".to_string(),
        }
    }
}
