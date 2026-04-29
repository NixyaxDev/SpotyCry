import type { ClientRequest, ServerResponse } from './protocol'

const DEFAULT_URL = 'ws://127.0.0.1:8080'

export async function sendWebSocketRequest<TData, TPayload = unknown>(
  request: ClientRequest<TPayload>,
  url = DEFAULT_URL,
): Promise<ServerResponse<TData>> {
  return new Promise((resolve, reject) => {
    const socket = new WebSocket(url)

    socket.addEventListener('open', () => {
      socket.send(JSON.stringify(request))
    })

    socket.addEventListener('message', (event) => {
      try {
        const response = JSON.parse(event.data) as ServerResponse<TData>
        resolve(response)
      } catch {
        reject(new Error('El servidor devolvió una respuesta inválida'))
      } finally {
        socket.close()
      }
    })

    socket.addEventListener('error', () => {
      reject(new Error('No se pudo conectar con el servidor WebSocket'))
    })
  })
}
