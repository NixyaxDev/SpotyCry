import { useState } from 'react'

type CreatePlaylistFormProps = {
  onSubmit: (name: string) => Promise<boolean>
  loading: boolean
  error: string | null
}

export function CreatePlaylistForm({
  onSubmit,
  loading,
  error,
}: CreatePlaylistFormProps) {
  const [name, setName] = useState('')
  const [localError, setLocalError] = useState<string | null>(null)

  return (
    <form className="playlist-form-card" onSubmit={handleSubmit}>
      <div className="playlist-form-copy">
        <p className="eyebrow">Crear playlist</p>
        <h3>Crea una nueva colección a partir de las canciones disponibles en el servidor.</h3>
      </div>

      <label className="playlist-form-field">
        <span>Nombre</span>
        <input
          type="text"
          value={name}
          onChange={(event) => {
            setName(event.target.value)
            if (localError) {
              setLocalError(null)
            }
          }}
          placeholder="Mi playlist"
          disabled={loading}
        />
      </label>

      {(localError || error) && (
        <p className="playlist-form-error">{localError ?? error}</p>
      )}

      <button type="submit" className="primary-button" disabled={loading}>
        {loading ? 'Creando...' : 'Crear playlist'}
      </button>
    </form>
  )

  async function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault()

    if (name.trim().length === 0) {
      setLocalError('El nombre de la playlist no puede estar vacío')
      return
    }

    const wasCreated = await onSubmit(name)

    if (wasCreated) {
      setName('')
      setLocalError(null)
    }
  }
}
