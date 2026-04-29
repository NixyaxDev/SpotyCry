# Protocolo WebSocket actual

## Propósito

El protocolo WebSocket de SpotiCry permite la comunicación entre el cliente web y el servidor Rust usando mensajes JSON.

El modelo actual soporta:

- consulta de catálogo
- búsqueda de canciones
- inicio de reproducción
- detención de reproducción

## Formato general de request

Todas las solicitudes siguen esta estructura:

```json
{
  "request_id": "req-001",
  "action": "nombre_accion",
  "payload": {}
}
```

### Campos

- `request_id`: identificador del request para correlacionar respuestas
- `action`: acción que el servidor debe ejecutar
- `payload`: datos específicos de la acción

## Formato general de success response

```json
{
  "request_id": "req-001",
  "status": "success",
  "data": {}
}
```

## Formato general de error response

```json
{
  "request_id": "req-001",
  "status": "error",
  "error": {
    "code": "ERROR_CODE",
    "message": "Readable message"
  }
}
```

## Acciones implementadas

### 1. `list_songs`

Request:

```json
{
  "request_id": "req-001",
  "action": "list_songs",
  "payload": {}
}
```

Respuesta exitosa:

```json
{
  "request_id": "req-001",
  "status": "success",
  "data": {
    "songs": [
      {
        "id": "song-001",
        "title": "christmas.mp3",
        "artist": null,
        "genre": null,
        "duration": null
      }
    ]
  }
}
```

Si no hay canciones:

```json
{
  "request_id": "req-001",
  "status": "success",
  "data": {
    "songs": []
  }
}
```

### 2. `search_songs`

Request:

```json
{
  "request_id": "req-002",
  "action": "search_songs",
  "payload": {
    "criteria": "title",
    "value": "christ"
  }
}
```

Comportamiento:

- solo soporta búsqueda por `title`
- ignora mayúsculas/minúsculas
- recorta espacios extra
- si el valor es vacío, devuelve todas las canciones

Error esperado si el criterio no es válido:

```json
{
  "request_id": "req-002",
  "status": "error",
  "error": {
    "code": "INVALID_SEARCH_CRITERIA",
    "message": "Only title search is supported in HU-07"
  }
}
```

### 3. `start_playback`

Request:

```json
{
  "request_id": "req-003",
  "action": "start_playback",
  "payload": {
    "song_id": "song-001"
  }
}
```

Respuesta inicial:

```json
{
  "request_id": "req-003",
  "status": "success",
  "data": {
    "stream_id": "stream-001",
    "song_id": "song-001",
    "title": "christmas.mp3",
    "mime_type": "audio/mpeg",
    "chunk_size": 65536
  }
}
```

Después de eso, el servidor envía eventos `audio_chunk`.

### 4. `stop_playback`

Request:

```json
{
  "request_id": "req-004",
  "action": "stop_playback",
  "payload": {
    "stream_id": "stream-001",
    "song_id": "song-001"
  }
}
```

Respuesta exitosa:

```json
{
  "request_id": "req-004",
  "status": "success",
  "data": {
    "stream_id": "stream-001",
    "song_id": "song-001",
    "stopped": true
  }
}
```

## Eventos de reproducción

Durante la reproducción, el servidor envía:

```json
{
  "type": "audio_chunk",
  "stream_id": "stream-001",
  "song_id": "song-001",
  "chunk_index": 0,
  "is_last": false,
  "bytes_base64": "..."
}
```

## Errores estructurados actuales

Errores definidos actualmente:

- `INVALID_JSON`
- `UNSUPPORTED_ACTION`
- `INTERNAL_ERROR`
- `INVALID_SEARCH_CRITERIA`
- `INVALID_PAYLOAD`
- `SONG_NOT_FOUND`
- `FILE_NOT_FOUND`
- `STREAM_ERROR`

## Observaciones de diseño

- El protocolo es simple y legible.
- Se prefirió JSON por facilidad de depuración y de integración con React.
- La correlación por `request_id` hace más claro el seguimiento de respuestas.
- Las respuestas tienen forma estable, lo que facilita el manejo en frontend.
