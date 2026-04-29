use std::sync::{Arc, Mutex};

use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use uuid::Uuid;

use crate::playback::active_streams::{
    register_stream, remove_stream, stream_song_id, ActiveStreams,
};
use crate::playback::stream_service::read_song_chunks;
use crate::playlists::{
    build_playlist_summary, filter_playlist_songs, parse_filter_criteria, parse_sort_criteria,
    parse_sort_direction, sort_playlist_songs, PlaylistLibrary, PlaylistLibraryError,
    PlaylistOperationError,
};
use crate::protocol::error::ErrorBody;
use crate::protocol::request::{
    ClientRequest, CreatePlaylistPayload, FilterPlaylistSongsPayload, PlaylistSongPayload,
    PlaylistSummaryPayload, SearchSongsPayload, SortPlaylistSongsPayload, StartPlaybackPayload,
    StopPlaybackPayload,
};
use crate::protocol::response::{
    CreatePlaylistData, ErrorResponse, ListPlaylistsData, ListSongsData, PlaylistData, PlaylistDto,
    PlaylistSummaryData, PlaylistSummaryDto, SongDto, StopPlaybackData, SuccessResponse,
};
use crate::songs::SongLibrary;
use crate::songs::SongLibraryError;

pub async fn handle_connection(
    mut websocket_stream: WebSocketStream<TcpStream>,
    client_address: String,
    song_library: Arc<Mutex<SongLibrary>>,
    playlist_library: Arc<Mutex<PlaylistLibrary>>,
    active_streams: ActiveStreams,
) {
    println!("👤 Cliente conectado: {}", client_address);
    let mut current_stream_id: Option<String> = None;

    while let Some(message_result) = websocket_stream.next().await {
        match message_result {
            Ok(Message::Text(text)) => {
                println!("📩 Mensaje recibido de {}: {}", client_address, text);

                if let Err(error) = handle_text_message(
                    &mut websocket_stream,
                    &text,
                    &song_library,
                    &playlist_library,
                    &active_streams,
                    &mut current_stream_id,
                )
                .await
                {
                    eprintln!(
                        "❌ Error procesando mensaje de {}: {}",
                        client_address, error
                    );
                    break;
                }
            }

            Ok(Message::Binary(bytes)) => {
                println!(
                    "📦 Mensaje binario recibido de {}: {} bytes",
                    client_address,
                    bytes.len()
                );
            }

            Ok(Message::Close(_)) => {
                println!("🔴 Cliente desconectado: {}", client_address);
                break;
            }

            Ok(Message::Ping(payload)) => {
                println!("🏓 Ping recibido de {}", client_address);

                if let Err(error) = websocket_stream.send(Message::Pong(payload)).await {
                    eprintln!("❌ Error enviando Pong a {}: {}", client_address, error);
                    break;
                }
            }

            Ok(Message::Pong(_)) => {
                println!("🏓 Pong recibido de {}", client_address);
            }

            Err(error) => {
                eprintln!("❌ Error en la conexión con {}: {}", client_address, error);
                break;
            }

            _ => {}
        }
    }

    cleanup_stream_state(&song_library, &active_streams, &mut current_stream_id);
    println!("🧹 Conexión finalizada: {}", client_address);
}

async fn handle_text_message(
    websocket_stream: &mut WebSocketStream<TcpStream>,
    text: &str,
    song_library: &Arc<Mutex<SongLibrary>>,
    playlist_library: &Arc<Mutex<PlaylistLibrary>>,
    active_streams: &ActiveStreams,
    current_stream_id: &mut Option<String>,
) -> Result<(), tokio_tungstenite::tungstenite::Error> {
    match serde_json::from_str::<ClientRequest>(text) {
        Ok(request) => {
            handle_request(
                websocket_stream,
                request,
                song_library,
                playlist_library,
                active_streams,
                current_stream_id,
            )
            .await
        }
        Err(_) => {
            let response =
                serialize_response(&ErrorResponse::new("unknown", ErrorBody::invalid_json()));
            websocket_stream.send(Message::Text(response)).await
        }
    }
}

async fn handle_request(
    websocket_stream: &mut WebSocketStream<TcpStream>,
    request: ClientRequest,
    song_library: &Arc<Mutex<SongLibrary>>,
    playlist_library: &Arc<Mutex<PlaylistLibrary>>,
    active_streams: &ActiveStreams,
    current_stream_id: &mut Option<String>,
) -> Result<(), tokio_tungstenite::tungstenite::Error> {
    match request.action.as_str() {
        "list_songs" => {
            let _ = &request.payload;

            let response = match song_library.lock() {
                Ok(library) => {
                    let songs = library
                        .song_summaries()
                        .into_iter()
                        .map(SongDto::from)
                        .collect();

                    serialize_response(&SuccessResponse::new(
                        request.request_id,
                        ListSongsData { songs },
                    ))
                }
                Err(_) => serialize_response(&ErrorResponse::new(
                    request.request_id,
                    ErrorBody::internal_error(),
                )),
            };

            websocket_stream.send(Message::Text(response)).await
        }
        "list_playlists" => {
            let response = match playlist_library.lock() {
                Ok(library) => {
                    let playlists = library
                        .playlists()
                        .into_iter()
                        .map(PlaylistDto::from)
                        .collect();

                    serialize_response(&SuccessResponse::new(
                        request.request_id,
                        ListPlaylistsData { playlists },
                    ))
                }
                Err(_) => serialize_response(&ErrorResponse::new(
                    request.request_id,
                    ErrorBody::internal_error(),
                )),
            };

            websocket_stream.send(Message::Text(response)).await
        }
        "create_playlist" => match serde_json::from_value::<CreatePlaylistPayload>(request.payload)
        {
            Ok(payload) => {
                let response = match playlist_library.lock() {
                    Ok(mut library) => match library.create_playlist(&payload.name) {
                        Ok(playlist) => serialize_response(&SuccessResponse::new(
                            request.request_id,
                            CreatePlaylistData {
                                playlist: PlaylistDto::from(playlist),
                            },
                        )),
                        Err(error) => serialize_response(&ErrorResponse::new(
                            request.request_id,
                            map_playlist_library_error(error),
                        )),
                    },
                    Err(_) => serialize_response(&ErrorResponse::new(
                        request.request_id,
                        ErrorBody::internal_error(),
                    )),
                };

                websocket_stream.send(Message::Text(response)).await
            }
            Err(_) => {
                let response = serialize_response(&ErrorResponse::new(
                    request.request_id,
                    ErrorBody::invalid_payload(),
                ));
                websocket_stream.send(Message::Text(response)).await
            }
        },
        "add_song_to_playlist" => {
            match serde_json::from_value::<PlaylistSongPayload>(request.payload) {
                Ok(payload)
                    if payload.playlist_id.trim().is_empty()
                        || payload.song_id.trim().is_empty() =>
                {
                    let response = serialize_response(&ErrorResponse::new(
                        request.request_id,
                        ErrorBody::invalid_payload(),
                    ));
                    websocket_stream.send(Message::Text(response)).await
                }
                Ok(payload) => {
                    let response = handle_add_song_to_playlist(
                        request.request_id,
                        payload,
                        song_library,
                        playlist_library,
                    );
                    websocket_stream.send(Message::Text(response)).await
                }
                Err(_) => {
                    let response = serialize_response(&ErrorResponse::new(
                        request.request_id,
                        ErrorBody::invalid_payload(),
                    ));
                    websocket_stream.send(Message::Text(response)).await
                }
            }
        }
        "remove_song_from_playlist" => {
            match serde_json::from_value::<PlaylistSongPayload>(request.payload) {
                Ok(payload)
                    if payload.playlist_id.trim().is_empty()
                        || payload.song_id.trim().is_empty() =>
                {
                    let response = serialize_response(&ErrorResponse::new(
                        request.request_id,
                        ErrorBody::invalid_payload(),
                    ));
                    websocket_stream.send(Message::Text(response)).await
                }
                Ok(payload) => {
                    let response = handle_remove_song_from_playlist(
                        request.request_id,
                        payload,
                        playlist_library,
                    );
                    websocket_stream.send(Message::Text(response)).await
                }
                Err(_) => {
                    let response = serialize_response(&ErrorResponse::new(
                        request.request_id,
                        ErrorBody::invalid_payload(),
                    ));
                    websocket_stream.send(Message::Text(response)).await
                }
            }
        }
        "filter_playlist_songs" => {
            match serde_json::from_value::<FilterPlaylistSongsPayload>(request.payload) {
                Ok(payload)
                    if payload.playlist_id.trim().is_empty()
                        || payload.criteria.trim().is_empty() =>
                {
                    let response = serialize_response(&ErrorResponse::new(
                        request.request_id,
                        ErrorBody::invalid_payload(),
                    ));
                    websocket_stream.send(Message::Text(response)).await
                }
                Ok(payload) => {
                    let response = handle_filter_playlist_songs(
                        request.request_id,
                        payload,
                        song_library,
                        playlist_library,
                    );
                    websocket_stream.send(Message::Text(response)).await
                }
                Err(_) => {
                    let response = serialize_response(&ErrorResponse::new(
                        request.request_id,
                        ErrorBody::invalid_payload(),
                    ));
                    websocket_stream.send(Message::Text(response)).await
                }
            }
        }
        "sort_playlist_songs" => {
            match serde_json::from_value::<SortPlaylistSongsPayload>(request.payload) {
                Ok(payload)
                    if payload.playlist_id.trim().is_empty()
                        || payload.criteria.trim().is_empty()
                        || payload.direction.trim().is_empty() =>
                {
                    let response = serialize_response(&ErrorResponse::new(
                        request.request_id,
                        ErrorBody::invalid_payload(),
                    ));
                    websocket_stream.send(Message::Text(response)).await
                }
                Ok(payload) => {
                    let response = handle_sort_playlist_songs(
                        request.request_id,
                        payload,
                        song_library,
                        playlist_library,
                    );
                    websocket_stream.send(Message::Text(response)).await
                }
                Err(_) => {
                    let response = serialize_response(&ErrorResponse::new(
                        request.request_id,
                        ErrorBody::invalid_payload(),
                    ));
                    websocket_stream.send(Message::Text(response)).await
                }
            }
        }
        "get_playlist_summary" => {
            match serde_json::from_value::<PlaylistSummaryPayload>(request.payload) {
                Ok(payload) if payload.playlist_id.trim().is_empty() => {
                    let response = serialize_response(&ErrorResponse::new(
                        request.request_id,
                        ErrorBody::invalid_payload(),
                    ));
                    websocket_stream.send(Message::Text(response)).await
                }
                Ok(payload) => {
                    let response = handle_get_playlist_summary(
                        request.request_id,
                        payload,
                        song_library,
                        playlist_library,
                    );
                    websocket_stream.send(Message::Text(response)).await
                }
                Err(_) => {
                    let response = serialize_response(&ErrorResponse::new(
                        request.request_id,
                        ErrorBody::invalid_payload(),
                    ));
                    websocket_stream.send(Message::Text(response)).await
                }
            }
        }
        "search_songs" => {
            let response = match serde_json::from_value::<SearchSongsPayload>(request.payload) {
                Ok(payload) => {
                    if payload.criteria.trim().to_lowercase() != "title" {
                        serialize_response(&ErrorResponse::new(
                            request.request_id,
                            ErrorBody::invalid_search_criteria(),
                        ))
                    } else {
                        match song_library.lock() {
                            Ok(library) => {
                                let songs = library
                                    .search_by_title(&payload.value)
                                    .into_iter()
                                    .map(SongDto::from)
                                    .collect();

                                serialize_response(&SuccessResponse::new(
                                    request.request_id,
                                    ListSongsData { songs },
                                ))
                            }
                            Err(_) => serialize_response(&ErrorResponse::new(
                                request.request_id,
                                ErrorBody::internal_error(),
                            )),
                        }
                    }
                }
                Err(_) => serialize_response(&ErrorResponse::new(
                    request.request_id,
                    ErrorBody::invalid_payload(),
                )),
            };

            websocket_stream.send(Message::Text(response)).await
        }
        "start_playback" => match serde_json::from_value::<StartPlaybackPayload>(request.payload) {
            Ok(payload) if payload.song_id.trim().is_empty() => {
                let response = serialize_response(&ErrorResponse::new(
                    request.request_id,
                    ErrorBody::invalid_payload(),
                ));
                websocket_stream.send(Message::Text(response)).await
            }
            Ok(payload) => {
                cleanup_stream_state(song_library, active_streams, current_stream_id);
                let playback_song_result = match song_library.lock() {
                    Ok(mut library) => match library.find_song(&payload.song_id) {
                        Some(song) => match library.set_active_song(&payload.song_id) {
                            Ok(_) => Ok(song),
                            Err(SongLibraryError::SongNotFound) => Err(ErrorBody::song_not_found()),
                            Err(_) => Err(ErrorBody::stream_error()),
                        },
                        None => Err(ErrorBody::song_not_found()),
                    },
                    Err(_) => Err(ErrorBody::internal_error()),
                };

                let playback_song = match playback_song_result {
                    Ok(song) => song,
                    Err(error) => {
                        return send_protocol_error(websocket_stream, request.request_id, error)
                            .await;
                    }
                };

                let stream_id = format!("stream-{}", Uuid::new_v4());
                register_stream(active_streams, stream_id.clone(), payload.song_id.clone());
                *current_stream_id = Some(stream_id.clone());

                let stream_result = read_song_chunks(&playback_song, stream_id.clone()).await;

                match stream_result {
                    Ok((start_data, chunk_messages)) => {
                        let initial_response = serialize_response(&SuccessResponse::new(
                            request.request_id,
                            start_data,
                        ));
                        websocket_stream
                            .send(Message::Text(initial_response))
                            .await?;

                        for chunk_message in chunk_messages {
                            websocket_stream.send(Message::Text(chunk_message)).await?;
                        }

                        Ok(())
                    }
                    Err(error_code) => {
                        cleanup_stream_state(song_library, active_streams, current_stream_id);
                        let error = match error_code.as_str() {
                            "FILE_NOT_FOUND" => ErrorBody::file_not_found(),
                            "STREAM_ERROR" => ErrorBody::stream_error(),
                            _ => ErrorBody::stream_error(),
                        };

                        let response =
                            serialize_response(&ErrorResponse::new(request.request_id, error));
                        websocket_stream.send(Message::Text(response)).await
                    }
                }
            }
            Err(_) => {
                let response = serialize_response(&ErrorResponse::new(
                    request.request_id,
                    ErrorBody::invalid_payload(),
                ));
                websocket_stream.send(Message::Text(response)).await
            }
        },
        "stop_playback" => match serde_json::from_value::<StopPlaybackPayload>(request.payload) {
            Ok(payload)
                if payload.song_id.trim().is_empty() || payload.stream_id.trim().is_empty() =>
            {
                let response = serialize_response(&ErrorResponse::new(
                    request.request_id,
                    ErrorBody::invalid_payload(),
                ));
                websocket_stream.send(Message::Text(response)).await
            }
            Ok(payload) => {
                let registered_song_id = stream_song_id(active_streams, &payload.stream_id);

                match registered_song_id {
                    Some(song_id) if song_id == payload.song_id => {
                        remove_stream(active_streams, &payload.stream_id);

                        if let Ok(mut library) = song_library.lock() {
                            library.clear_active_song(&payload.song_id);
                        }

                        if current_stream_id.as_deref() == Some(payload.stream_id.as_str()) {
                            *current_stream_id = None;
                        }

                        let response = serialize_response(&SuccessResponse::new(
                            request.request_id,
                            StopPlaybackData {
                                stream_id: payload.stream_id,
                                song_id: payload.song_id,
                                stopped: true,
                            },
                        ));

                        websocket_stream.send(Message::Text(response)).await
                    }
                    _ => {
                        let response = serialize_response(&ErrorResponse::new(
                            request.request_id,
                            ErrorBody::playback_not_found(),
                        ));
                        websocket_stream.send(Message::Text(response)).await
                    }
                }
            }
            Err(_) => {
                let response = serialize_response(&ErrorResponse::new(
                    request.request_id,
                    ErrorBody::invalid_payload(),
                ));
                websocket_stream.send(Message::Text(response)).await
            }
        },
        _ => {
            let response = serialize_response(&ErrorResponse::new(
                request.request_id,
                ErrorBody::unsupported_action(&request.action),
            ));
            websocket_stream.send(Message::Text(response)).await
        }
    }
}

fn serialize_response<T: serde::Serialize>(response: &T) -> String {
    serde_json::to_string(response)
        .unwrap_or_else(|_| "{\"request_id\":\"unknown\",\"status\":\"error\",\"error\":{\"code\":\"INTERNAL_ERROR\",\"message\":\"Could not serialize response\"}}".to_string())
}

async fn send_protocol_error(
    websocket_stream: &mut WebSocketStream<TcpStream>,
    request_id: String,
    error: ErrorBody,
) -> Result<(), tokio_tungstenite::tungstenite::Error> {
    let response = serialize_response(&ErrorResponse::new(request_id, error));
    websocket_stream.send(Message::Text(response)).await
}

fn cleanup_stream_state(
    song_library: &Arc<Mutex<SongLibrary>>,
    active_streams: &ActiveStreams,
    current_stream_id: &mut Option<String>,
) {
    if let Some(stream_id) = current_stream_id.take() {
        if let Some(song_id) = remove_stream(active_streams, &stream_id) {
            if let Ok(mut library) = song_library.lock() {
                library.clear_active_song(&song_id);
            }
        }
    }
}

fn handle_add_song_to_playlist(
    request_id: String,
    payload: PlaylistSongPayload,
    song_library: &Arc<Mutex<SongLibrary>>,
    playlist_library: &Arc<Mutex<PlaylistLibrary>>,
) -> String {
    let song_exists = match song_library.lock() {
        Ok(library) => library.has_song(&payload.song_id),
        Err(_) => {
            return serialize_response(&ErrorResponse::new(request_id, ErrorBody::internal_error()))
        }
    };

    if !song_exists {
        return serialize_response(&ErrorResponse::new(request_id, ErrorBody::song_not_found()));
    }

    match playlist_library.lock() {
        Ok(mut library) => {
            match library.add_song_to_playlist(&payload.playlist_id, &payload.song_id) {
                Ok(playlist) => serialize_response(&SuccessResponse::new(
                    request_id,
                    PlaylistData {
                        playlist: PlaylistDto::from(playlist),
                    },
                )),
                Err(error) => serialize_response(&ErrorResponse::new(
                    request_id,
                    map_playlist_library_error(error),
                )),
            }
        }
        Err(_) => serialize_response(&ErrorResponse::new(request_id, ErrorBody::internal_error())),
    }
}

fn handle_remove_song_from_playlist(
    request_id: String,
    payload: PlaylistSongPayload,
    playlist_library: &Arc<Mutex<PlaylistLibrary>>,
) -> String {
    match playlist_library.lock() {
        Ok(mut library) => {
            match library.remove_song_from_playlist(&payload.playlist_id, &payload.song_id) {
                Ok(playlist) => serialize_response(&SuccessResponse::new(
                    request_id,
                    PlaylistData {
                        playlist: PlaylistDto::from(playlist),
                    },
                )),
                Err(error) => serialize_response(&ErrorResponse::new(
                    request_id,
                    map_playlist_library_error(error),
                )),
            }
        }
        Err(_) => serialize_response(&ErrorResponse::new(request_id, ErrorBody::internal_error())),
    }
}

fn handle_filter_playlist_songs(
    request_id: String,
    payload: FilterPlaylistSongsPayload,
    song_library: &Arc<Mutex<SongLibrary>>,
    playlist_library: &Arc<Mutex<PlaylistLibrary>>,
) -> String {
    let criteria = match parse_filter_criteria(&payload.criteria) {
        Ok(criteria) => criteria,
        Err(PlaylistOperationError::InvalidFilterCriteria) => {
            return serialize_response(&ErrorResponse::new(
                request_id,
                ErrorBody::invalid_filter_criteria(),
            ))
        }
        Err(_) => {
            return serialize_response(&ErrorResponse::new(request_id, ErrorBody::internal_error()))
        }
    };

    let playlist = match playlist_library.lock() {
        Ok(library) => match library.find_playlist(&payload.playlist_id) {
            Some(playlist) => playlist,
            None => {
                return serialize_response(&ErrorResponse::new(
                    request_id,
                    ErrorBody::playlist_not_found(),
                ))
            }
        },
        Err(_) => {
            return serialize_response(&ErrorResponse::new(request_id, ErrorBody::internal_error()))
        }
    };

    match song_library.lock() {
        Ok(library) => {
            let playlist_songs = library.song_summaries_by_ids(&playlist.song_ids);
            let songs = filter_playlist_songs(&playlist_songs, criteria, &payload.value)
                .into_iter()
                .map(SongDto::from)
                .collect();

            serialize_response(&SuccessResponse::new(request_id, ListSongsData { songs }))
        }
        Err(_) => serialize_response(&ErrorResponse::new(request_id, ErrorBody::internal_error())),
    }
}

fn handle_sort_playlist_songs(
    request_id: String,
    payload: SortPlaylistSongsPayload,
    song_library: &Arc<Mutex<SongLibrary>>,
    playlist_library: &Arc<Mutex<PlaylistLibrary>>,
) -> String {
    let criteria = match parse_sort_criteria(&payload.criteria) {
        Ok(criteria) => criteria,
        Err(PlaylistOperationError::InvalidSortCriteria) => {
            return serialize_response(&ErrorResponse::new(
                request_id,
                ErrorBody::invalid_sort_criteria(),
            ))
        }
        Err(_) => {
            return serialize_response(&ErrorResponse::new(request_id, ErrorBody::internal_error()))
        }
    };

    let direction = match parse_sort_direction(&payload.direction) {
        Ok(direction) => direction,
        Err(PlaylistOperationError::InvalidSortDirection) => {
            return serialize_response(&ErrorResponse::new(
                request_id,
                ErrorBody::invalid_sort_direction(),
            ))
        }
        Err(_) => {
            return serialize_response(&ErrorResponse::new(request_id, ErrorBody::internal_error()))
        }
    };

    let playlist = match playlist_library.lock() {
        Ok(library) => match library.find_playlist(&payload.playlist_id) {
            Some(playlist) => playlist,
            None => {
                return serialize_response(&ErrorResponse::new(
                    request_id,
                    ErrorBody::playlist_not_found(),
                ))
            }
        },
        Err(_) => {
            return serialize_response(&ErrorResponse::new(request_id, ErrorBody::internal_error()))
        }
    };

    match song_library.lock() {
        Ok(library) => {
            let playlist_songs = library.song_summaries_by_ids(&playlist.song_ids);
            let songs = sort_playlist_songs(&playlist_songs, criteria, direction)
                .into_iter()
                .map(SongDto::from)
                .collect();

            serialize_response(&SuccessResponse::new(request_id, ListSongsData { songs }))
        }
        Err(_) => serialize_response(&ErrorResponse::new(request_id, ErrorBody::internal_error())),
    }
}

fn handle_get_playlist_summary(
    request_id: String,
    payload: PlaylistSummaryPayload,
    song_library: &Arc<Mutex<SongLibrary>>,
    playlist_library: &Arc<Mutex<PlaylistLibrary>>,
) -> String {
    let playlist = match playlist_library.lock() {
        Ok(library) => match library.find_playlist(&payload.playlist_id) {
            Some(playlist) => playlist,
            None => {
                return serialize_response(&ErrorResponse::new(
                    request_id,
                    ErrorBody::playlist_not_found(),
                ))
            }
        },
        Err(_) => {
            return serialize_response(&ErrorResponse::new(request_id, ErrorBody::internal_error()))
        }
    };

    match song_library.lock() {
        Ok(library) => {
            let summary =
                build_playlist_summary(&library.song_summaries_by_ids(&playlist.song_ids));

            serialize_response(&SuccessResponse::new(
                request_id,
                PlaylistSummaryData {
                    summary: PlaylistSummaryDto::from(summary),
                },
            ))
        }
        Err(_) => serialize_response(&ErrorResponse::new(request_id, ErrorBody::internal_error())),
    }
}

fn map_playlist_library_error(error: PlaylistLibraryError) -> ErrorBody {
    match error {
        PlaylistLibraryError::InvalidName => ErrorBody::invalid_playlist_name(),
        PlaylistLibraryError::AlreadyExists => ErrorBody::playlist_already_exists(),
        PlaylistLibraryError::PlaylistNotFound => ErrorBody::playlist_not_found(),
        PlaylistLibraryError::SongAlreadyInPlaylist => ErrorBody::song_already_in_playlist(),
        PlaylistLibraryError::SongNotInPlaylist => ErrorBody::song_not_in_playlist(),
    }
}
