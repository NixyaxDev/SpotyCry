# Catálogo de canciones y CLI de administración

## Propósito

El catálogo de canciones es el núcleo compartido entre:

- el servidor WebSocket
- el CLI del administrador
- el flujo de reproducción

La implementación actual usa memoria compartida y no utiliza base de datos.

## Estado compartido

El estado principal del catálogo se almacena en:

```rust
Arc<Mutex<SongLibrary>>
```

Esto permite que varios componentes accedan al mismo catálogo de forma segura:

- el CLI puede agregar, listar, activar o eliminar canciones
- el servidor puede listar canciones, buscarlas y reproducirlas
- la lógica de borrado puede impedir eliminar canciones activas

## Estructura principal

La entidad principal es `Song`.

Campos relevantes:

- `id`
- `title`
- `artist`
- `album`
- `genre`
- `duration`
- `file_path`
- `is_active`

`SongLibrary` administra:

- almacenamiento en memoria
- asignación incremental de IDs
- validaciones
- búsqueda por título
- activación/desactivación
- eliminación protegida

## Registro de canciones

Cuando el administrador usa:

```text
add <ruta>
```

el sistema realiza este flujo:

1. valida que la ruta exista
2. valida que sea un archivo
3. valida que la extensión sea `.mp3` o `.wav`
4. normaliza la ruta con `canonicalize`
5. construye metadata básica
6. verifica duplicados
7. registra la canción en memoria

También existe un modo de carga por carpeta:

```text
add-dir <carpeta>
```

En ese caso el servidor:

1. valida que la ruta exista
2. valida que sea una carpeta
3. recorre los archivos contenidos en esa carpeta
4. detecta únicamente archivos `.mp3` y `.wav`
5. intenta registrar cada canción usando la misma lógica de `add <ruta>`
6. reporta cuáles canciones fueron agregadas y cuáles se omitieron

Esto permite cargar en lote todas las canciones válidas encontradas en una carpeta local del servidor.

### Manejo de metadata

Actualmente se usa una estrategia simple:

- `title = nombre del archivo`
- `artist = None`
- `album = None`
- `genre = None`
- `duration = None`

Esto fue suficiente para cumplir el alcance funcional sin agregar complejidad innecesaria.

## Duplicados

Una canción se considera duplicada si cumple al menos una de estas condiciones:

- misma ruta de archivo
- mismo título y mismo tamaño de archivo

Esto evita registros repetidos en el catálogo.

## Eliminación protegida

La eliminación en SpotiCry:

- elimina la canción del catálogo en memoria
- no elimina el archivo físico del disco

Además, una canción activa no puede eliminarse.

Si el usuario intenta borrar una canción en reproducción, el sistema responde con un error controlado.

## CLI disponible

Comandos implementados:

- `help`
- `list`
- `search`
- `add <path>`
- `add-dir <folder-path>`
- `delete <song-id>`
- `active`
- `active <song-id>`
- `exit`

La presencia de `add-dir` es importante porque permite demostrar no solo registro individual de canciones, sino también una estrategia práctica de carga masiva desde una carpeta.

## Resultado técnico

La solución deja una base clara para futuras extensiones:

- persistencia en disco o base de datos
- playlists reales
- metadata enriquecida
- control de reproducción más complejo

Al mismo tiempo, mantiene el diseño simple y adecuado para un proyecto universitario.
