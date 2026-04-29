# Comportamiento actual del cliente web

## Propósito

El frontend de SpotiCry está construido con:

- React
- Vite
- TypeScript

Su comportamiento actual depende de datos reales entregados por el servidor Rust.

## Principio importante

El frontend no debe inventar canciones.

Por eso:

- no mantiene un catálogo mock de canciones
- no reproduce archivos locales del frontend
- no renderiza cards falsas de canciones

La pantalla de canciones depende únicamente de lo que retorna el servidor.

## Carga inicial de canciones

Al entrar a la vista de canciones:

1. el hook `useSongs` abre un WebSocket
2. envía la acción `list_songs`
3. espera la respuesta del servidor
4. actualiza la interfaz con el catálogo recibido

Si no hay canciones, el frontend muestra un estado vacío.

## Búsqueda de canciones

Cuando el usuario escribe en la barra de búsqueda:

1. el frontend toma el valor escrito
2. si el valor está vacío, consulta `list_songs`
3. si el valor tiene contenido, envía `search_songs`
4. renderiza el resultado recibido

La búsqueda no se hace localmente sobre un arreglo fijo; se delega al servidor.

## Reproducción

Cuando el usuario pulsa reproducir:

1. el cliente envía `start_playback`
2. recibe metadata inicial
3. recibe eventos `audio_chunk`
4. reconstruye el audio en memoria
5. crea un `Blob URL`
6. reproduce con `<audio>`

## Sincronización de controles

Actualmente existen dos lugares visibles de control:

- botón de la tabla de canciones
- reproductor inferior

La lógica fue corregida para que ambos reflejen el mismo estado:

- si la canción está sonando, muestran acción de detener
- si la canción no está sonando, muestran acción de reproducir
- si el audio se detiene desde el reproductor inferior, también se limpia el estado del servidor

## Estados de interfaz implementados

La pantalla de canciones maneja:

- `loading`
- `error`
- `empty`
- `results`

Esto hace que la experiencia sea más clara para el usuario y más fácil de explicar en una demostración.

## Resultado técnico

El frontend actual:

- consume datos reales del backend
- hace búsqueda real sobre el servidor
- reproduce audio real enviado por WebSocket
- evita inconsistencias básicas entre controles visibles

Esto es valioso para la sección de resultados porque demuestra integración real entre frontend y backend.
