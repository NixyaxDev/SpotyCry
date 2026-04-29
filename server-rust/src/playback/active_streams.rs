use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type ActiveStreams = Arc<Mutex<HashMap<String, String>>>;

pub fn register_stream(active_streams: &ActiveStreams, stream_id: String, song_id: String) {
    if let Ok(mut streams) = active_streams.lock() {
        streams.insert(stream_id, song_id);
    }
}

pub fn remove_stream(active_streams: &ActiveStreams, stream_id: &str) -> Option<String> {
    active_streams
        .lock()
        .ok()
        .and_then(|mut streams| streams.remove(stream_id))
}

pub fn stream_song_id(active_streams: &ActiveStreams, stream_id: &str) -> Option<String> {
    active_streams
        .lock()
        .ok()
        .and_then(|streams| streams.get(stream_id).cloned())
}
