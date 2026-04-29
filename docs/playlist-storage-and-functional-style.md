# Gestión de playlists en el servidor y estilo funcional

## Decisión de almacenamiento

Para este proyecto se decidió que las playlists se administren y almacenen **en el servidor**, no en el cliente.

La decisión actual es:

- las canciones físicas viven en el servidor
- la metadata de canciones vive en memoria del servidor
- las playlists también viven en memoria del servidor
- el cliente solo envía solicitudes y renderiza respuestas del servidor

## Justificación de la decisión

Guardar playlists en el servidor es la opción más conveniente para este alcance porque:

- mantiene un único source of truth
- evita sincronización compleja entre múltiples clientes
- evita que cada cliente tenga una versión distinta de las playlists
- simplifica las operaciones de agregar, eliminar, filtrar y ordenar
- deja la lógica de negocio en Rust, como pide la consigna

Si las playlists vivieran en el cliente:

- habría que sincronizarlas constantemente
- cada navegador podría quedar con un estado distinto
- la lógica funcional quedaría más fragmentada entre frontend y backend

## Qué significa esto en la práctica

El servidor es responsable de:

- crear playlists
- agregar canciones a playlists
- remover canciones de playlists
- filtrar canciones dentro de playlists
- ordenar canciones dentro de playlists
- calcular resúmenes

El cliente:

- envía requests WebSocket
- recibe respuestas JSON
- actualiza la UI con datos reales del servidor

## Estado compartido

Actualmente esto queda centralizado en:

- [`server-rust/src/state/mod.rs`](/Users/adriana/Documents/GitHub/SpotyCry/server-rust/src/state/mod.rs)

con una estructura `AppState` que agrupa:

- `songs`
- `playlists`
- `active_streams`

## Restricciones funcionales aplicadas a playlists

La consigna pide un enfoque funcional específicamente para playlists. La implementación sigue ese criterio en estos puntos:

### 1. Transformaciones puras sobre `Playlist`

Las operaciones principales trabajan sobre una playlist de entrada y retornan una nueva playlist:

- agregar canción
- remover canción

Eso vive en:

- [`server-rust/src/playlists/playlist_operations.rs`](/Users/adriana/Documents/GitHub/SpotyCry/server-rust/src/playlists/playlist_operations.rs)

### 2. Uso de `map`

Se usa `map` para:

- transformar resultados a DTOs
- reemplazar una playlist actualizada dentro de la colección sin modificar elemento por elemento

### 3. Uso de `filter`

Se usa `filter` para:

- remover canciones de playlists
- filtrar canciones por criterio dentro de una playlist

### 4. Uso de `fold`

Se usa `fold` para construir el resumen estadístico:

- cantidad de canciones
- duración conocida total
- cantidad de duraciones desconocidas

### 5. Uso de closures

Se usan closures en:

- validaciones de duplicados
- filtros de canciones
- reemplazo funcional de playlists
- construcción de resúmenes

## Mutabilidad evitada donde fue práctico

El contenedor `PlaylistLibrary` sigue siendo un estado en memoria y por eso necesita mutación controlada para persistir cambios mientras el servidor está corriendo.

Sin embargo, dentro de esa estructura se redujo mutabilidad innecesaria:

- al crear playlists se reemplaza la colección con una nueva versión
- al actualizar una playlist se reconstruye la colección con `map(...)`
- las operaciones de negocio sobre una playlist retornan nuevos valores

## Conclusión

La implementación actual cumple razonablemente con la consigna porque:

- las playlists son administradas por el servidor
- el cliente no es la fuente de verdad
- las operaciones exclusivas de playlists usan un enfoque funcional en Rust
- la arquitectura sigue siendo simple y adecuada para el contexto universitario
