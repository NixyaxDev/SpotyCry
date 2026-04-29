# Arquitectura recomendada y estructura actual de SpotiCry

## Objetivo

Este documento resume la distribución de carpetas recomendada para este proyecto y explica por qué la estructura actual es adecuada para seguir creciendo sin perder claridad.

## Principio general

SpotiCry está dividido por responsabilidades funcionales y no por archivos sueltos.

La idea es que cada módulo tenga una responsabilidad clara:

- el backend resuelve catálogo, playlists, protocolo y reproducción
- el frontend resuelve presentación, interacción y consumo del servidor

## Backend

### Estructura actual recomendada

```text
server-rust/src/
├── cli/
├── network/
├── playback/
├── playlists/
├── protocol/
├── songs/
└── state/
```

### Justificación por módulo

#### `cli/`

Contiene el CLI local del administrador.

Debe vivir separado porque:

- usa `std::io`
- corre fuera del flujo del cliente web
- interactúa con el mismo estado compartido

#### `network/`

Contiene:

- servidor WebSocket
- manejo de conexiones
- routing de requests

Debe mantenerse separado porque es la frontera de comunicación entre cliente y servidor.

#### `playback/`

Contiene:

- streaming de audio
- chunks
- streams activos

Esto ayuda a no mezclar reproducción con catálogo o protocolo.

#### `playlists/`

Contiene:

- modelo de playlist
- librería en memoria
- operaciones funcionales
- resumen estadístico

Es un buen lugar para mantener el enfoque funcional aislado y testeable.

#### `protocol/`

Contiene:

- payloads
- responses
- errores estructurados

Esto hace que el contrato JSON quede explícito y fácil de documentar.

#### `songs/`

Contiene:

- modelo de canción
- catálogo
- validaciones
- búsqueda

Es el dominio principal del proyecto.

#### `state/`

Contiene `AppState`.

Esto permite pasar una sola estructura compartida al servidor y a futuros módulos en vez de crecer con listas de parámetros largas y frágiles.

## Frontend

### Estructura actual recomendada

```text
frontend/src/
├── app/
├── api/
├── components/
├── features/
├── shared/
├── types/
└── views/
```

### Justificación por módulo

#### `app/`

Contiene la composición principal y el view model general.

Es útil para que `App.tsx` no se vuelva un archivo con demasiada lógica.

#### `api/`

Contiene:

- cliente WebSocket
- tipos del protocolo compartido

Esto separa claramente transporte de UI.

#### `components/`

Se usa para piezas compartidas de layout o presentación general:

- sidebar
- topbar
- player bar

#### `features/`

Cada dominio del sistema tiene su propia feature:

- `songs`
- `playback`
- `playlists`

Esta es la estructura ideal para crecer sin mezclar reglas de negocio distintas.

#### `shared/`

Contiene utilidades reutilizables como:

- constantes
- formatters
- mappers comunes

#### `types/`

Tipos UI compartidos entre múltiples pantallas.

#### `views/`

Pantallas principales del sistema:

- songs
- playlists
- playlist detail
- now playing

## Beneficios de esta arquitectura

- menor acoplamiento
- mejor mantenibilidad
- mejor legibilidad
- más facilidad para documentar el sistema
- más facilidad para extender el proyecto

## Qué no se buscó hacer

No se buscó:

- sobreingeniería
- introducir demasiadas capas innecesarias
- convertir el proyecto en una arquitectura empresarial pesada

La meta fue una arquitectura clara, académicamente defendible y compatible con el alcance del curso.
