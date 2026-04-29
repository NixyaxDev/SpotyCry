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
        <p className="eyebrow">Create Playlist</p>
        <h3>Build a new collection from the songs available on the server.</h3>
      </div>

      <label className="playlist-form-field">
        <span>Name</span>
        <input
          type="text"
          value={name}
          onChange={(event) => {
            setName(event.target.value)
            if (localError) {
              setLocalError(null)
            }
          }}
          placeholder="My Playlist"
          disabled={loading}
        />
      </label>

      {(localError || error) && (
        <p className="playlist-form-error">{localError ?? error}</p>
      )}

      <button type="submit" className="primary-button" disabled={loading}>
        {loading ? 'Creating...' : 'Create playlist'}
      </button>
    </form>
  )

  async function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault()

    if (name.trim().length === 0) {
      setLocalError('Playlist name cannot be empty')
      return
    }

    const wasCreated = await onSubmit(name)

    if (wasCreated) {
      setName('')
      setLocalError(null)
    }
  }
}
