# Instituto Tecnológico de Costa Rica

## Ingeniería en Computación

### Lenguajes de Programación

# Especificación Completa del Protocolo de Comunicación

## Proyecto: SpotiCry

------------------------------------------------------------------------

## 1. Introducción

Este documento define de manera formal y técnica el protocolo de
comunicación utilizado en el sistema SpotiCry, el cual consiste en una
arquitectura cliente-servidor donde:

-   El **cliente web** (React + Vite + TypeScript) envía solicitudes.
-   El **servidor en Rust** procesa dichas solicitudes.
-   Se mantiene una comunicación persistente mediante WebSocket.

Este documento sirve tanto como: - 📘 Documentación técnica del
sistema - 🤖 Guía para agentes de programación - 🧩 Contrato entre
cliente y servidor

------------------------------------------------------------------------

## 2. Justificación de la Arquitectura

El enunciado original propone el uso de sockets TCP/IP. Sin embargo:

-   Los navegadores web no permiten conexiones TCP directas.
-   Se requiere comunicación bidireccional persistente.

Por lo tanto, se utiliza **WebSocket**, que: - Funciona sobre TCP -
Permite comunicación en tiempo real - Es compatible con navegadores

------------------------------------------------------------------------

## 3. Principios del Protocolo

El protocolo está diseñado bajo los siguientes principios:

-   📦 Simplicidad → uso de JSON
-   🔄 Consistencia → estructura uniforme
-   🔍 Trazabilidad → uso de `request_id`
-   ⚡ Escalabilidad → preparado para múltiples clientes
-   🧱 Modularidad → acciones independientes

------------------------------------------------------------------------

## 4. Modelo de Comunicación

    Cliente → Request → Servidor
    Servidor → Response → Cliente
    Servidor → Eventos (streaming) → Cliente

Tipos de mensajes: 1. Request (cliente → servidor) 2. Response (servidor
→ cliente) 3. Event (servidor → cliente, ej: audio)

------------------------------------------------------------------------

## 5. Estructura de Mensajes

### 5.1 Request

``` json
{
  "request_id": "string",
  "action": "string",
  "payload": {}
}
```

### 5.2 Response

``` json
{
  "request_id": "string",
  "status": "success",
  "data": {}
}
```

### 5.3 Error

``` json
{
  "request_id": "string",
  "status": "error",
  "error": {
    "code": "string",
    "message": "string"
  }
}
```

------------------------------------------------------------------------

## 6. Acciones Definidas

### 6.1 list_songs

Lista todas las canciones disponibles.

**Response:**

``` json
{
  "songs": [
    {
      "id": "string",
      "title": "string",
      "artist": "string",
      "genre": "string",
      "duration_seconds": 0
    }
  ]
}
```

------------------------------------------------------------------------

### 6.2 search_songs

Permite búsqueda por: - title - artist - genre

------------------------------------------------------------------------

### 6.3 start_playback

Inicia reproducción de una canción.

**Response:**

``` json
{
  "stream_id": "string",
  "chunk_size": 4096
}
```

------------------------------------------------------------------------

### 6.4 audio_chunk (Evento)

``` json
{
  "type": "audio_chunk",
  "stream_id": "string",
  "chunk_index": 0,
  "is_last": false,
  "bytes_base64": "..."
}
```

------------------------------------------------------------------------

### 6.5 stop_playback

Detiene reproducción activa.

------------------------------------------------------------------------

### 6.6 create_playlist

Crea una nueva playlist.

------------------------------------------------------------------------

### 6.7 add_song_to_playlist

Agrega canción a playlist.

------------------------------------------------------------------------

### 6.8 filter_playlist

Filtra playlist usando programación funcional.

------------------------------------------------------------------------

## 7. Modelo de Datos

### Song

``` json
{
  "id": "string",
  "title": "string",
  "artist": "string",
  "genre": "string",
  "duration_seconds": number
}
```

### Playlist

``` json
{
  "id": "string",
  "name": "string",
  "songs": []
}
```

------------------------------------------------------------------------

## 8. Manejo de Errores

  Código               Descripción
  -------------------- ------------------------
  INVALID_ACTION       Acción inválida
  INVALID_PAYLOAD      Datos incorrectos
  SONG_NOT_FOUND       Canción no encontrada
  PLAYLIST_NOT_FOUND   Playlist no encontrada
  SONG_IN_PLAYBACK     Canción en uso
  INTERNAL_ERROR       Error interno

------------------------------------------------------------------------

## 9. Concurrencia

El servidor debe:

-   Manejar múltiples clientes simultáneamente
-   Procesar requests de forma independiente
-   Gestionar reproducción concurrente

------------------------------------------------------------------------

## 10. Flujo de Ejemplo

1.  Cliente envía `search_songs`
2.  Servidor responde con lista
3.  Cliente envía `start_playback`
4.  Servidor envía múltiples `audio_chunk`
5.  Cliente reproduce audio

------------------------------------------------------------------------

## 11. Consideraciones para Implementación

### Cliente

-   Manejar WebSocket
-   Buffer de audio
-   Estado de reproducción

### Servidor

-   Parseo de JSON
-   Manejo de conexiones
-   Streaming eficiente

------------------------------------------------------------------------

## 12. Conclusión

Este protocolo define una base sólida, clara y extensible para la
comunicación en el sistema SpotiCry, permitiendo tanto su implementación
como su evolución futura.
