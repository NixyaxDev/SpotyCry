use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AudioChunkEvent {
    #[serde(rename = "type")]
    pub event_type: &'static str,
    pub stream_id: String,
    pub song_id: String,
    pub chunk_index: usize,
    pub is_last: bool,
    pub bytes_base64: String,
}

impl AudioChunkEvent {
    pub fn new(
        stream_id: String,
        song_id: String,
        chunk_index: usize,
        is_last: bool,
        bytes_base64: String,
    ) -> Self {
        Self {
            event_type: "audio_chunk",
            stream_id,
            song_id,
            chunk_index,
            is_last,
            bytes_base64,
        }
    }
}
