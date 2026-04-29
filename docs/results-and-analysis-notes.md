# Notas para resultados generados y análisis de resultados

## Resultados generados

Los resultados más claros que ya se pueden reportar en el proyecto son los siguientes.

### 1. Catálogo compartido funcional

Se logró construir un catálogo de canciones administrado en memoria y compartido entre:

- el CLI del servidor
- el servidor WebSocket
- el flujo de reproducción

Esto permitió demostrar operaciones reales sobre un mismo estado sin duplicar estructuras entre componentes.

### 2. Registro real de canciones desde archivos locales

El sistema permite registrar canciones desde archivos `.mp3` y `.wav`, validando:

- existencia del archivo
- tipo de archivo
- duplicados

Como resultado, el catálogo no depende de datos ficticios ni de ejemplos hardcodeados.

### 3. Protocolo WebSocket estable

Se definió un protocolo JSON simple y consistente para:

- listar canciones
- buscar canciones
- iniciar reproducción
- detener reproducción

El uso de mensajes estructurados con `request_id`, `status`, `data` y `error` facilita pruebas, depuración e integración con el frontend.

### 4. Cliente web conectado a datos reales

El frontend consume exclusivamente datos del servidor, lo cual demuestra una integración cliente-servidor efectiva.

Esto es importante porque evita resultados engañosos basados en datos simulados.

### 5. Reproducción funcional de audio

Se logró reproducir audio real desde el servidor usando:

- WebSocket
- envío de chunks codificados en base64
- reconstrucción del archivo en el navegador
- reproducción con `<audio>`

Este resultado demuestra que la arquitectura implementada no solo administra catálogo, sino que también ejecuta una reproducción real de archivos.

### 6. Control de detención y liberación de estado

Con la implementación de `stop_playback`, el sistema libera el estado activo de reproducción y vuelve a permitir operaciones como eliminación.

Esto mejora la consistencia del sistema y reduce errores lógicos visibles.

## Análisis de resultados

## 1. Simplicidad vs complejidad técnica

Uno de los principales resultados del proyecto es que se logró una solución funcional manteniendo una arquitectura relativamente simple.

En lugar de introducir base de datos, colas complejas o streaming avanzado, se priorizó:

- claridad
- facilidad de implementación
- facilidad de explicación
- estabilidad en pruebas manuales

Esto fue adecuado para el contexto universitario.

## 2. Uso de memoria compartida

El uso de `Arc<Mutex<SongLibrary>>` resultó suficiente para coordinar el catálogo entre el CLI y el servidor WebSocket.

Aunque no es una solución de alta escala, fue correcta para un entorno controlado y permitió demostrar concurrencia segura sin sobreingeniería.

## 3. Fortalezas del protocolo

El protocolo JSON fue una buena decisión para esta fase porque:

- es legible
- es fácil de depurar
- se integra naturalmente con React
- permite extender acciones sin romper el modelo general

Esto favorece la mantenibilidad del sistema.

## 4. Limitaciones del playback actual

La reproducción implementada funciona correctamente para el alcance del proyecto, pero tiene límites claros:

- el audio se reconstruye completo antes de reproducirse
- no hay streaming progresivo avanzado
- no se usa `MediaSource`

Esto significa que la solución favorece robustez y simplicidad sobre sofisticación.

## 5. Coherencia entre backend y frontend

Un punto importante en el análisis es que varias historias obligaron a sincronizar cuidadosamente el estado entre servidor y cliente.

Por ejemplo:

- una canción activa no debe poder eliminarse
- si el cliente detiene reproducción, el servidor debe liberar estado
- los controles visuales deben coincidir con el estado real del audio

Resolver estos puntos aportó madurez al sistema, porque hizo visible la diferencia entre “que algo funcione” y “que funcione de forma consistente”.

## 6. Valor de las decisiones documentadas

El proyecto también fue generando valor documental:

- decisiones de alcance
- justificación de simplificaciones
- estado de historias parcialmente cubiertas o completamente cubiertas

Esto facilita la defensa del proyecto y la redacción del informe técnico.

## Conclusión sugerida

Una conclusión razonable para el informe sería:

> SpotiCry logró implementar un sistema cliente-servidor funcional para administración, consulta, búsqueda y reproducción básica de canciones, priorizando simplicidad arquitectónica, consistencia entre componentes y una integración real entre frontend y backend. Aunque algunas decisiones sacrifican sofisticación técnica, el resultado final cumple el alcance académico propuesto y deja una base clara para futuras mejoras.
