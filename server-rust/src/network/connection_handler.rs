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
use crate::protocol::error::ErrorBody;
use crate::protocol::request::{
    ClientRequest, SearchSongsPayload, StartPlaybackPayload, StopPlaybackPayload,
};
use crate::protocol::response::{
    ErrorResponse, ListSongsData, SongDto, StopPlaybackData, SuccessResponse,
};
use crate::songs::SongLibraryError;
use crate::songs::SongLibrary;

pub async fn handle_connection(
    mut websocket_stream: WebSocketStream<TcpStream>,
    client_address: String,
    song_library: Arc<Mutex<SongLibrary>>,
    active_streams: ActiveStreams,
) {
    println!("👤 Cliente conectado: {}", client_address);
    let mut current_stream_id: Option<String> = None;

    while let Some(message_result) = websocket_stream.next().await {
        match message_result {
            Ok(Message::Text(text)) => {
                println!("📩 Mensaje recibido de {}: {}", client_address, text);

                if let Err(error) =
                    handle_text_message(
                        &mut websocket_stream,
                        &text,
                        &song_library,
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
                    eprintln!(
                        "❌ Error enviando Pong a {}: {}",
                        client_address, error
                    );
                    break;
                }
            }

            Ok(Message::Pong(_)) => {
                println!("🏓 Pong recibido de {}", client_address);
            }

            Err(error) => {
                eprintln!(
                    "❌ Error en la conexión con {}: {}",
                    client_address, error
                );
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
    active_streams: &ActiveStreams,
    current_stream_id: &mut Option<String>,
) -> Result<(), tokio_tungstenite::tungstenite::Error> {
    match serde_json::from_str::<ClientRequest>(text) {
        Ok(request) => {
            handle_request(
                websocket_stream,
                request,
                song_library,
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
                    Ok(mut library) => {
                        match library.find_song(&payload.song_id) {
                            Some(song) => match library.set_active_song(&payload.song_id) {
                                Ok(_) => Ok(song),
                                Err(SongLibraryError::SongNotFound) => {
                                    Err(ErrorBody::song_not_found())
                                }
                                Err(_) => Err(ErrorBody::stream_error()),
                            },
                            None => Err(ErrorBody::song_not_found()),
                        }
                    }
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
                        let initial_response =
                            serialize_response(&SuccessResponse::new(request.request_id, start_data));
                        websocket_stream.send(Message::Text(initial_response)).await?;

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
            Ok(payload) if payload.song_id.trim().is_empty() || payload.stream_id.trim().is_empty() => {
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
