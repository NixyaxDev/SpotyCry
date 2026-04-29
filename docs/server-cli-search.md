# Búsqueda desde el CLI del servidor

## Objetivo

Además de la búsqueda desde el cliente web, el servidor ahora permite buscar canciones directamente desde el CLI del administrador.

Esto es útil para:

- validar rápidamente qué canciones están cargadas
- revisar metadata desde el servidor
- probar búsquedas sin depender del frontend

## Comando disponible

```text
search
```

## Cómo funciona

Cuando el administrador escribe:

```text
search
```

el CLI pregunta explícitamente qué criterio quiere usar antes de ejecutar la búsqueda.

Flujo:

1. el CLI pide el criterio
2. el administrador responde uno de estos valores:
   - `title`
   - `artist`
   - `album`
   - `genre`
3. el CLI pide el valor de búsqueda
4. el servidor ejecuta la búsqueda usando ese criterio
5. se imprimen los resultados encontrados

## Ejemplo de uso

```text
> search
Search criterion (title/artist/album/genre): artist
Search value: adele
Found 2 song(s) using 'artist' as search criterion:
- song-001 | Hello | artist: Adele | album: 25 | genre: Pop | duration: 295s
- song-004 | Easy On Me | artist: Adele | album: 30 | genre: Pop | duration: 224s
```

## Reglas de búsqueda

La búsqueda desde CLI comparte la misma lógica base que la búsqueda del servidor para el cliente web:

- es case-insensitive
- recorta espacios extra
- si el valor está vacío, devuelve todas las canciones

### Comportamiento por criterio

- `title`: busca coincidencias parciales en cualquier parte del título
- `artist`: busca por prefijo en cualquiera de las palabras del artista
- `album`: busca por prefijo sobre el nombre del álbum
- `genre`: exige coincidencia exacta normalizada

## Criterios válidos

Se soportan:

- `title`
- `artist`
- `album`
- `genre`

Si se ingresa otro criterio, el CLI responde con error controlado.

## Relación con el backend

El CLI no implementa una búsqueda separada.

Internamente reutiliza la lógica del catálogo de canciones en:

- [`server-rust/src/songs/library.rs`](/Users/adriana/Documents/GitHub/SpotyCry/server-rust/src/songs/library.rs)

Esto ayuda a mantener consistencia entre:

- búsqueda en frontend
- búsqueda desde WebSocket
- búsqueda local desde el servidor

## Motivación de diseño

Se eligió que el servidor **pregunte primero el criterio** porque eso hace el flujo más claro para un entorno de terminal y evita ambigüedad en comandos con múltiples parámetros.

También deja la interacción más alineada con la consigna:

- el usuario define el criterio
- el sistema ejecuta la búsqueda según ese criterio
