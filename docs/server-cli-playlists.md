# Administración de playlists desde la consola del servidor

## Objetivo

El servidor ahora permite administrar playlists también desde el CLI local del administrador.

Esto mantiene paridad funcional razonable con el frontend y permite:

- crear playlists
- listar playlists
- ver canciones dentro de una playlist
- agregar canciones a playlists
- eliminar canciones de playlists
- filtrar canciones dentro de playlists
- ordenar canciones dentro de playlists
- ver el resumen de una playlist

## Comandos disponibles

### Listar playlists

```text
playlist list
```

### Crear playlist

```text
playlist create My Favorites
```

### Ver canciones de una playlist

```text
playlist songs playlist-001
```

### Agregar canción a una playlist

```text
playlist add-song playlist-001 song-001
```

### Eliminar canción de una playlist

```text
playlist remove-song playlist-001 song-001
```

### Filtrar canciones dentro de una playlist

```text
playlist filter playlist-001 title love
playlist filter playlist-001 artist adele
playlist filter playlist-001 genre pop
```

### Ordenar canciones dentro de una playlist

```text
playlist sort playlist-001 title asc
playlist sort playlist-001 artist desc
playlist sort playlist-001 duration asc
```

### Ver resumen de una playlist

```text
playlist summary playlist-001
```

## Observaciones

- Las playlists siguen viviendo en memoria del servidor.
- La consola reutiliza la misma lógica de negocio que usa el frontend por WebSocket.
- Agregar o eliminar canciones desde la consola afecta el mismo estado compartido que ve el cliente web.

## Relación con la consigna

Esto ayuda a cumplir que el servidor también haga administración de playlists y no dependa exclusivamente del frontend para esas operaciones.
