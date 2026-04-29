# Guía de Ejecución - SpotiCry

## Propósito

Este documento explica cómo ejecutar el proyecto **SpotiCry**, tanto en backend como en frontend, y qué comandos básicos se pueden usar para probar sus funcionalidades principales.

## Requisitos previos

Antes de correr el proyecto se recomienda tener instalado:

- `Rust` y `cargo`
- `Node.js`
- `npm`

## Estructura general

El proyecto está dividido en dos partes:

- `server-rust/` → servidor en Rust
- `frontend/` → cliente web en React + Vite + TypeScript

## 1. Cómo correr el servidor

Abrir una terminal en la raíz del proyecto y ejecutar:

```bash
cd server-rust
cargo run
```

Si solo se quiere validar compilación:

```bash
cargo check
```

## 2. Cómo correr el frontend

En otra terminal:

```bash
cd frontend
npm install
npm run dev
```

Si solo se quiere validar compilación:

```bash
npm run build
```

## 3. Orden recomendado de ejecución

Para probar el sistema completo se recomienda:

1. levantar primero el servidor Rust
2. dejar abierta esa terminal
3. levantar luego el frontend
4. abrir la aplicación web en el navegador

## 4. Comandos principales del CLI del servidor

Cuando el servidor está corriendo, el administrador puede usar la consola local para gestionar canciones y playlists.

### Comandos generales

```text
help
list
search
add ./src/songs/christmas.mp3
add-dir ./src/songs
delete song-001
active
active song-001
exit
```

### Qué hace cada uno

- `help` → muestra todos los comandos disponibles
- `list` → muestra las canciones cargadas en memoria
- `search` → inicia una búsqueda interactiva por criterio
- `add <ruta>` → agrega una canción desde archivo local
- `add-dir <carpeta>` → agrega todas las canciones soportadas de una carpeta local
- `delete <song-id>` → elimina una canción si no está activa
- `active` → muestra la canción activa actual
- `active <song-id>` → marca una canción como activa
- `exit` → cierra el CLI del servidor

## 5. Búsqueda desde la consola del servidor

El comando:

```text
search
```

pregunta primero el criterio:

- `title`
- `artist`
- `album`
- `genre`

Ejemplo:

```text
search
Search criterion (title/artist/album/genre): title
Search value: christmas
```

## 6. Comandos de playlists desde la consola

```text
playlist list
playlist create My Favorites
playlist songs playlist-001
playlist add-song playlist-001 song-001
playlist remove-song playlist-001 song-001
playlist filter playlist-001 title christmas
playlist filter playlist-001 artist adele
playlist filter playlist-001 genre pop
playlist sort playlist-001 title asc
playlist sort playlist-001 artist desc
playlist sort playlist-001 duration asc
playlist summary playlist-001
```

## 7. Flujo básico de prueba manual

Un flujo recomendado para demostrar el sistema es:

### Paso 1: cargar canciones

```text
add ./src/songs/christmas.mp3
add ./src/songs/navidad.mp3
add-dir ./src/songs
list
```

### Paso 2: buscar canciones

```text
search
```

### Paso 3: crear una playlist

```text
playlist create Demo Playlist
playlist list
```

### Paso 4: agregar canciones a la playlist

```text
playlist add-song playlist-001 song-001
playlist songs playlist-001
playlist summary playlist-001
```

### Paso 5: probar el frontend

Con el frontend abierto:

- ir a la vista de canciones
- buscar canciones
- reproducir una canción
- probar `Now Playing`
- crear una playlist
- reproducir una playlist

## 8. Comandos útiles de validación

### Backend

```bash
cd server-rust
cargo check
```

### Frontend

```bash
cd frontend
npm run build
```

## 9. Notas importantes

- Las canciones físicas viven en el servidor.
- La metadata, playlists y estado de reproducción viven en memoria.
- Si el servidor se reinicia, las playlists se pierden porque no hay persistencia.
- El cliente web depende de que el servidor esté corriendo.
- La reproducción usa buffer local solo para la canción actual.

## 10. Cómo referenciar este documento en el informe final

En el informe final se puede mencionar algo como:

> Para detalles de instalación, ejecución y comandos de prueba del sistema, ver la “Guía de Ejecución - SpotiCry”.

También se puede citar dentro de anexos como:

- **Anexo C - Guía de ejecución del proyecto**
