# HU-13 a HU-18 - Estado del módulo de playlists

## Propósito de este documento

Este archivo resume lo realizado desde la implementación inicial de playlists hasta completar:

- HU-13: creación de playlists
- HU-14: agregar canción a playlist
- HU-15: remover canción de playlist
- HU-16: filtrar canciones dentro de playlist
- HU-17: ordenar canciones de playlist
- HU-18: resumen de playlist con `fold`

Sirve como apoyo para:

- justificar historias marcadas como terminadas
- redactar avances del proyecto
- alimentar el informe final

## Qué se implementó

### HU-13

Se implementó la creación de playlists desde el cliente web.

Incluye:

- modelo `Playlist`
- `PlaylistLibrary` en memoria
- validación de nombre vacío
- prevención de duplicados case-insensitive
- acción WebSocket `create_playlist`
- render real de playlists en frontend

### HU-14

Se implementó agregar canciones a playlists.

Reglas cumplidas:

- una canción puede pertenecer a múltiples playlists
- una canción no puede repetirse dentro de la misma playlist
- la canción debe existir en el catálogo
- la playlist debe existir

### HU-15

Se implementó remover una canción de una playlist sin borrar la canción del catálogo general.

Reglas cumplidas:

- la playlist debe existir
- la canción debe pertenecer a esa playlist
- el catálogo global no se modifica al remover desde playlist

### HU-16

Se implementó filtrado de canciones dentro de una playlist por:

- `title`
- `artist`
- `genre`

El filtrado:

- es case-insensitive
- soporta coincidencias parciales
- devuelve solo canciones que pertenecen a la playlist seleccionada

### HU-17

Se implementó ordenamiento de canciones dentro de una playlist por:

- `title`
- `artist`
- `duration`

Direcciones soportadas:

- `asc`
- `desc`

### HU-18

Se implementó un resumen de playlist con:

- cantidad total de canciones
- duración total conocida en segundos
- cantidad de canciones con duración desconocida

El cálculo se hace con `fold`.

## Requisito funcional en Rust

La consigna pedía un enfoque funcional. En esta parte del proyecto se siguió de esta manera:

- transformaciones puras sobre `Playlist`
- uso de `iter().any(...)`
- uso de `filter(...).cloned().collect()`
- uso de closures
- uso de `fold(...)` para el resumen

### Ejemplos importantes

Agregar canción:

- se toma una playlist
- se construye una nueva playlist con un `Vec<String>` actualizado
- no se muta directamente el valor de entrada

Remover canción:

- se usa `filter` para devolver una nueva colección de `song_ids`

Resumen:

- se usa `fold` para acumular estadísticas

Ordenamiento:

- se ordena una copia temporal local del vector
- no se muta el estado persistido de la playlist

## Decisión importante sobre ordenamiento

Rust requiere mutación para ordenar un `Vec`.

Para mantener el espíritu funcional del proyecto se tomó esta decisión:

- no se muta la playlist original
- se crea una copia temporal de canciones
- se ordena esa copia
- se devuelve el resultado al cliente

Esto cumple el objetivo funcional sin comprometer el estado original.

## Estado actual de frontend

El frontend ya no usa playlists hardcodeadas.

Actualmente:

- lista playlists reales del servidor
- crea playlists reales
- agrega canciones a playlists reales
- remueve canciones de playlists reales
- filtra y ordena resultados solicitando datos al servidor
- muestra resumen calculado por backend

## Estado actual de backend

El backend ahora separa claramente:

- estado compartido en `AppState`
- catálogo de canciones
- playlists en memoria
- reproducción
- protocolo WebSocket

Esto deja una base más limpia para crecimiento futuro.

## Conclusión

HU-13 a HU-18 pueden considerarse implementadas en el alcance actual del proyecto.

La solución:

- cumple la funcionalidad requerida
- mantiene datos en memoria
- evita sobreingeniería
- usa un estilo funcional razonable en Rust
- conserva compatibilidad con el resto del sistema ya implementado
