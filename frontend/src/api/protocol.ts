export interface ClientRequest<T = unknown> {
  request_id: string
  action: string
  payload: T
}

export interface ServerSuccessResponse<T = unknown> {
  request_id: string
  status: 'success'
  data: T
}

export interface ServerErrorResponse {
  request_id: string
  status: 'error'
  error: {
    code: string
    message: string
  }
}

export type ServerResponse<T = unknown> =
  | ServerSuccessResponse<T>
  | ServerErrorResponse

export interface StartPlaybackPayload {
  song_id: string
}

export interface CreatePlaylistPayload {
  name: string
}
