# Flujo de reproducción y manejo de estado

## Propósito

La reproducción actual de SpotiCry permite que el cliente seleccione una canción, la reciba desde el servidor por WebSocket y la reproduzca en el navegador sin usar archivos hardcodeados en frontend.

## Flujo general

El flujo actual es:

1. el usuario selecciona una canción
2. el frontend envía `start_playback`
3. el servidor valida que la canción exista
4. el servidor marca la canción como activa
5. el servidor abre el archivo local
6. el servidor lo lee en chunks
7. el servidor envía esos chunks codificados en base64
8. el frontend reconstruye la canción en memoria
9. el navegador reproduce la canción usando `<audio>`

## Estado activo de reproducción

El proyecto maneja dos niveles de estado relacionados con playback:

### 1. Estado del catálogo

Cada canción tiene:

```rust
is_active: bool
```

Este campo sirve para:

- indicar cuál canción está en reproducción
- impedir su eliminación mientras está activa

### 2. Estado de streams

Además del catálogo, el servidor mantiene:

```rust
Arc<Mutex<HashMap<String, String>>>
```

La relación almacenada es:

- `stream_id -> song_id`

Esto permite:

- saber qué stream corresponde a qué canción
- detener reproducción de forma explícita
- limpiar estado cuando se cierra la conexión

## Inicio de reproducción

Al iniciar reproducción:

1. se limpia cualquier stream anterior asociado a la conexión
2. se busca la canción en `SongLibrary`
3. se marca como activa
4. se genera un `stream_id`
5. se registra en `active_streams`
6. se envía la metadata inicial al cliente
7. se envían los eventos `audio_chunk`

## Detención de reproducción

La historia HU-11 agregó una detención explícita.

Cuando el cliente envía `stop_playback`:

1. el servidor valida `stream_id` y `song_id`
2. elimina el stream de `active_streams`
3. limpia el estado activo de la canción
4. responde con éxito

Además, si la conexión se cierra, el servidor también intenta limpiar el stream asociado.

## Buffer local en el cliente

La canción actual se almacena temporalmente en memoria del navegador.

El enfoque actual usa:

- `Blob`
- `URL.createObjectURL(...)`
- `<audio controls>`

Esto cumple un buffer local simple para la canción actual.

## Seek local

Gracias a que el audio queda reconstruido en memoria:

- el usuario puede adelantar
- el usuario puede retroceder
- el usuario puede repetir navegación interna sobre la misma canción cargada

## Límite actual de la implementación

La solución actual no usa `MediaSource`.

Eso significa que:

- primero se reciben los chunks
- luego se arma el `Blob`
- luego se reproduce

Por lo tanto:

- no hay streaming progresivo avanzado
- no hay seek sobre datos parcialmente descargados

## Decisiones importantes

### Decisión 1: usar `Blob URL`

Se eligió por:

- simplicidad
- estabilidad
- facilidad de integración con React

### Decisión 2: detener reproducción explícitamente

Se implementó `stop_playback` para:

- liberar estado del servidor
- evitar falsos positivos de canciones “activas”
- permitir eliminar una canción después de detenerla

### Decisión 3: una sola canción activa

El diseño actual asume una canción activa a la vez dentro del catálogo.

Esto simplifica:

- la lógica del CLI
- la eliminación protegida
- el comportamiento visible del reproductor

## Resultado técnico

La implementación actual es suficiente para:

- demostrar reproducción end-to-end
- demostrar stop controlado
- justificar el buffer local de la canción actual

Y además deja una base extensible para:

- playlists reales
- colas de reproducción
- streaming progresivo futuro
- sincronización multiusuario más sofisticada
