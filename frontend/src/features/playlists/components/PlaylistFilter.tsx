type PlaylistFilterProps = {
  criteria: 'title' | 'artist' | 'genre'
  value: string
  onCriteriaChange: (criteria: 'title' | 'artist' | 'genre') => void
  onValueChange: (value: string) => void
  onApply: () => Promise<void>
  onReset: () => void
  loading: boolean
}

export function PlaylistFilter({
  criteria,
  value,
  onCriteriaChange,
  onValueChange,
  onApply,
  onReset,
  loading,
}: PlaylistFilterProps) {
  return (
    <div className="playlist-control-card">
      <p className="eyebrow">Filtrar canciones</p>
      <div className="playlist-inline-form">
        <select
          value={criteria}
          onChange={(event) =>
            onCriteriaChange(event.target.value as 'title' | 'artist' | 'genre')
          }
          disabled={loading}
        >
          <option value="title">Título</option>
          <option value="artist">Artista</option>
          <option value="genre">Género</option>
        </select>
        <input
          type="text"
          value={value}
          onChange={(event) => onValueChange(event.target.value)}
          placeholder="Buscar en la playlist..."
          disabled={loading}
        />
      </div>
      <div className="playlist-control-actions">
        <button
          type="button"
          className="primary-button"
          onClick={() => {
            void onApply()
          }}
          disabled={loading}
        >
          Aplicar filtro
        </button>
        <button type="button" className="secondary-button" onClick={onReset} disabled={loading}>
          Restablecer
        </button>
      </div>
    </div>
  )
}
