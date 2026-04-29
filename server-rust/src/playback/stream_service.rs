use std::path::Path;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::playback::audio_chunk::AudioChunkEvent;
use crate::protocol::response::StartPlaybackData;
use crate::songs::Song;

pub const CHUNK_SIZE: usize = 65_536;

pub async fn read_song_chunks(
    song: &Song,
    stream_id: String,
) -> Result<(StartPlaybackData, Vec<String>), String> {
    let mime_type = mime_type_for_path(&song.file_path)?;
    let mut file = File::open(&song.file_path)
        .await
        .map_err(|_| "FILE_NOT_FOUND".to_string())?;

    let mut buffer = vec![0_u8; CHUNK_SIZE];
    let mut chunk_index = 0_usize;
    let mut serialized_events = Vec::new();

    loop {
        let bytes_read = file
            .read(&mut buffer)
            .await
            .map_err(|_| "STREAM_ERROR".to_string())?;

        if bytes_read == 0 {
            if chunk_index == 0 {
                let event = AudioChunkEvent::new(
                    stream_id.clone(),
                    song.id.clone(),
                    0,
                    true,
                    String::new(),
                );
                serialized_events
                    .push(serde_json::to_string(&event).map_err(|_| "STREAM_ERROR".to_string())?);
            }
            break;
        }

        let is_last = bytes_read < CHUNK_SIZE;
        let encoded_bytes = STANDARD.encode(&buffer[..bytes_read]);
        let event = AudioChunkEvent::new(
            stream_id.clone(),
            song.id.clone(),
            chunk_index,
            is_last,
            encoded_bytes,
        );

        serialized_events
            .push(serde_json::to_string(&event).map_err(|_| "STREAM_ERROR".to_string())?);
        chunk_index += 1;

        if is_last {
            break;
        }
    }

    Ok((
        StartPlaybackData {
            stream_id,
            song_id: song.id.clone(),
            title: song.title.clone(),
            mime_type,
            chunk_size: CHUNK_SIZE,
        },
        serialized_events,
    ))
}

fn mime_type_for_path(file_path: &str) -> Result<String, String> {
    let extension = Path::new(file_path)
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_lowercase())
        .ok_or_else(|| "STREAM_ERROR".to_string())?;

    match extension.as_str() {
        "mp3" => Ok("audio/mpeg".to_string()),
        "wav" => Ok("audio/wav".to_string()),
        _ => Err("STREAM_ERROR".to_string()),
    }
}
