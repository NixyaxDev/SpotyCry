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
      <p className="eyebrow">Filter Songs</p>
      <div className="playlist-inline-form">
        <select
          value={criteria}
          onChange={(event) =>
            onCriteriaChange(event.target.value as 'title' | 'artist' | 'genre')
          }
          disabled={loading}
        >
          <option value="title">Title</option>
          <option value="artist">Artist</option>
          <option value="genre">Genre</option>
        </select>
        <input
          type="text"
          value={value}
          onChange={(event) => onValueChange(event.target.value)}
          placeholder="Search in playlist..."
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
          Apply filter
        </button>
        <button type="button" className="secondary-button" onClick={onReset} disabled={loading}>
          Reset
        </button>
      </div>
    </div>
  )
}
