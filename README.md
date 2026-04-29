# SpotiCry

Sistema cliente-servidor de música desarrollado como proyecto universitario usando **Rust**, **Tokio**, **WebSocket**, **React**, **Vite** y **TypeScript**.

## Qué hace hoy

SpotiCry ya incluye:

- servidor WebSocket concurrente en Rust
- CLI local de administración para el servidor
- catálogo compartido de canciones en memoria
- registro de canciones desde archivos `.mp3` y `.wav`
- búsqueda y listado de canciones por WebSocket
- reproducción básica con streaming por chunks
- detención explícita de reproducción
- buffer local de la canción actual en el cliente
- creación y gestión funcional de playlists

## Arquitectura general

### Backend

El backend vive en [`server-rust`](/Users/adriana/Documents/GitHub/SpotyCry/server-rust) y está organizado por dominios:

```text
server-rust/src/
├── cli/          # CLI administrativo local
├── network/      # servidor WebSocket y manejo de conexiones
├── playback/     # streaming y estado de reproducción
├── playlists/    # modelo, operaciones y resumen de playlists
├── protocol/     # requests, responses y errores JSON
├── songs/        # catálogo de canciones y validaciones
└── state/        # AppState compartido entre módulos
```

### Frontend

El frontend vive en [`frontend`](/Users/adriana/Documents/GitHub/SpotyCry/frontend) y está organizado por features:

```text
frontend/src/
├── app/          # composición principal y view model
├── api/          # cliente WebSocket y tipos de protocolo
├── components/   # layout y UI compartida
├── features/     # songs, playlists, playback
├── shared/       # constantes y formatters reutilizables
├── types/        # tipos UI compartidos
└── views/        # pantallas principales
```

## Tecnologías

- Rust
- Tokio
- tokio-tungstenite
- Serde / Serde JSON
- React
- Vite
- TypeScript

## Cómo ejecutar el proyecto

### Backend

```bash
cd server-rust
cargo run
```

### Frontend

```bash
cd frontend
npm install
npm run dev
```

## Comandos del CLI del servidor

Ejemplos:

```text
help
list
add ./src/songs/christmas.mp3
delete song-001
active
active song-001
exit
```

## Acciones WebSocket principales

Actualmente el servidor soporta:

- `list_songs`
- `search_songs`
- `start_playback`
- `stop_playback`
- `list_playlists`
- `create_playlist`
- `add_song_to_playlist`
- `remove_song_from_playlist`
- `filter_playlist_songs`
- `sort_playlist_songs`
- `get_playlist_summary`

## Estado del proyecto

El proyecto usa almacenamiento **en memoria** para metadata, playlists y estado de reproducción.

Eso significa:

- las canciones físicas viven en el servidor
- las playlists existen mientras el servidor esté encendido
- no hay base de datos
- no hay almacenamiento en la nube

## Documentación adicional

La documentación de apoyo vive en [`docs/`](/Users/adriana/Documents/GitHub/SpotyCry/docs).

Archivos recomendados para empezar:

- [docs/README.md](/Users/adriana/Documents/GitHub/SpotyCry/docs/README.md)
- [docs/architecture-overview.md](/Users/adriana/Documents/GitHub/SpotyCry/docs/architecture-overview.md)
- [docs/results-and-analysis-notes.md](/Users/adriana/Documents/GitHub/SpotyCry/docs/results-and-analysis-notes.md)
- [HU-12-buffer-local-status.md](/Users/adriana/Documents/GitHub/SpotyCry/HU-12-buffer-local-status.md)
- [HU-13-to-HU-18-playlists-status.md](/Users/adriana/Documents/GitHub/SpotyCry/HU-13-to-HU-18-playlists-status.md)

## Notas

- Se priorizó claridad y mantenibilidad sobre sobreingeniería.
- El enfoque actual es adecuado para el alcance académico del proyecto.
- La estructura modular deja una base clara para futuras mejoras como persistencia, playlists avanzadas o streaming progresivo más sofisticado.
