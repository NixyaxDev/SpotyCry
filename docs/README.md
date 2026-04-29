# Documentación de apoyo

Esta carpeta reúne notas técnicas y funcionales del proyecto **SpotiCry** para facilitar:

- la redacción del informe final
- la sección de resultados generados
- la sección de análisis de resultados
- la justificación de decisiones de diseño

## Documentos incluidos

- [informe-final-spoticry.md](/Users/adriana/Documents/GitHub/SpotyCry/docs/informe-final-spoticry.md)
  Documento principal en formato de informe académico, con requisitos, análisis técnico, resultados, conclusiones y sugerencias de diagramas.

- [guia-ejecucion-spoticry.md](/Users/adriana/Documents/GitHub/SpotyCry/docs/guia-ejecucion-spoticry.md)
  Documento de apoyo con instrucciones para correr el proyecto, comandos del servidor y flujo recomendado de prueba.

- [song-catalog-and-cli.md](/Users/adriana/Documents/GitHub/SpotyCry/docs/song-catalog-and-cli.md)
  Describe cómo se administra el catálogo de canciones, cómo funciona el CLI y cómo se comparte el estado con el servidor.

- [websocket-protocol.md](/Users/adriana/Documents/GitHub/SpotyCry/docs/websocket-protocol.md)
  Resume las acciones WebSocket implementadas, el formato de request/response y los errores estructurados.

- [server-cli-search.md](/Users/adriana/Documents/GitHub/SpotyCry/docs/server-cli-search.md)
  Explica la búsqueda interactiva desde el CLI del servidor y los criterios soportados.

- [server-cli-playlists.md](/Users/adriana/Documents/GitHub/SpotyCry/docs/server-cli-playlists.md)
  Resume los comandos de playlists disponibles desde la consola del servidor.

- [playback-flow-and-state.md](/Users/adriana/Documents/GitHub/SpotyCry/docs/playback-flow-and-state.md)
  Explica el flujo de reproducción, el manejo de streams activos, la lógica de inicio y detención, y el buffer local del cliente.

- [frontend-client-behavior.md](/Users/adriana/Documents/GitHub/SpotyCry/docs/frontend-client-behavior.md)
  Resume cómo el frontend consume el servidor, cómo busca canciones y cómo sincroniza los controles de reproducción.

- [results-and-analysis-notes.md](/Users/adriana/Documents/GitHub/SpotyCry/docs/results-and-analysis-notes.md)
  Contiene ideas redactables para la sección de resultados generados y análisis de resultados.

- [architecture-overview.md](/Users/adriana/Documents/GitHub/SpotyCry/docs/architecture-overview.md)
  Explica la arquitectura recomendada del proyecto y la razón detrás de la distribución actual de carpetas.

- [playlist-storage-and-functional-style.md](/Users/adriana/Documents/GitHub/SpotyCry/docs/playlist-storage-and-functional-style.md)
  Documenta por qué las playlists se administran en el servidor y cómo se aplicó el estilo funcional en Rust.

- [web-now-playing-buffer.md](/Users/adriana/Documents/GitHub/SpotyCry/docs/web-now-playing-buffer.md)
  Explica cómo el navegador mantiene en buffer solo la canción actual y cómo quedó conectada la vista `Now Playing`.

- [web-playback-queue.md](/Users/adriana/Documents/GitHub/SpotyCry/docs/web-playback-queue.md)
  Documenta la cola local de reproducción, el avance entre canciones y la reproducción de playlists desde la web.

## Nota

Además de estos documentos, ya existe el archivo:

- [HU-12-buffer-local-status.md](/Users/adriana/Documents/GitHub/SpotyCry/HU-12-buffer-local-status.md)
- [HU-13-to-HU-18-playlists-status.md](/Users/adriana/Documents/GitHub/SpotyCry/HU-13-to-HU-18-playlists-status.md)

Ese archivo mantiene una justificación puntual de la HU-12 y puede reutilizarse como anexo o evidencia de alcance.
