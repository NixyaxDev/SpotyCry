export interface StartPlaybackData {
  stream_id: string
  song_id: string
  title: string
  mime_type: string
  chunk_size: number
}

export interface AudioChunkEvent {
  type: 'audio_chunk'
  stream_id: string
  song_id: string
  chunk_index: number
  is_last: boolean
  bytes_base64: string
}

export interface StopPlaybackData {
  stream_id: string
  song_id: string
  stopped: boolean
}
