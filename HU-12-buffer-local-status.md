# HU-12 - Buffer local de canción en cliente

## Estado

HU-12 puede considerarse **terminada con implementación simple y estable**.

La solución actual cumple el objetivo funcional esperado para el proyecto universitario:

- solo se bufferiza la canción actual
- el cliente no descarga todo el catálogo
- el audio se almacena temporalmente en memoria del navegador
- el usuario puede adelantar y retroceder dentro de la canción cargada
- el buffer se limpia al cambiar de canción o detener reproducción

## Qué ya cumple la implementación actual

La implementación actual usa:

- WebSocket para recibir chunks de audio desde el servidor
- reconstrucción del audio en el cliente usando `Blob`
- reproducción mediante `URL.createObjectURL(...)`
- un elemento HTML `<audio>` con controles nativos

Esto permite que:

- solo la canción seleccionada se descargue y mantenga en memoria
- el usuario pueda usar seek hacia adelante y hacia atrás
- no existan archivos de audio hardcodeados en el frontend
- los recursos locales se limpien cuando cambia la canción o se detiene la reproducción

## Alcance real de la solución actual

La solución implementa un **buffer local completo de la canción actual**, no un buffering progresivo avanzado.

El flujo actual es:

1. el cliente solicita `start_playback`
2. el servidor envía la canción en chunks
3. el frontend recibe todos los chunks
4. el frontend arma un `Blob`
5. se genera un `Blob URL`
6. el `<audio>` reproduce esa canción

Con este enfoque:

- el seek funciona correctamente sobre la canción ya cargada
- solo la canción en turno queda bufferizada
- la limpieza del buffer local es simple y controlada

## Qué no se implementó

No se implementó:

- buffering progresivo mientras el audio ya está sonando
- `MediaSource API`
- seek sobre datos parcialmente descargados

Esto fue una decisión intencional para mantener la solución:

- simple
- estable
- fácil de explicar
- adecuada para el alcance del curso

## Checklist de HU-12

- [x] Solo la canción actual se bufferiza
- [x] El cliente puede adelantar y retroceder en la canción cargada
- [x] Se usa almacenamiento temporal en memoria
- [x] Se usa `Blob URL`
- [x] Se limpia el buffer al cambiar de canción
- [x] Se limpia el buffer al detener la reproducción
- [x] No se descarga todo el catálogo
- [ ] Se implementa buffering progresivo durante reproducción
- [ ] Se implementa `MediaSource`

## Justificación para marcarla como terminada

Para este proyecto universitario, HU-12 puede declararse como terminada porque el requerimiento principal era contar con un buffer local de la canción actual que permitiera hacer seek sin descargar todo el catálogo.

La implementación actual cumple eso de forma clara:

- mantiene solo la canción activa en memoria
- permite adelantar y retroceder libremente dentro de esa canción
- limpia recursos cuando deja de usarse

Si en una fase futura se necesitara una experiencia más avanzada de streaming, el siguiente paso natural sería migrar de `Blob URL` a `MediaSource`, pero eso no es necesario para considerar HU-12 cumplida en el alcance actual.
