type PlaylistSortProps = {
  criteria: 'title' | 'artist' | 'duration'
  direction: 'asc' | 'desc'
  onCriteriaChange: (criteria: 'title' | 'artist' | 'duration') => void
  onDirectionChange: (direction: 'asc' | 'desc') => void
  onApply: () => Promise<void>
  onReset: () => void
  loading: boolean
}

export function PlaylistSort({
  criteria,
  direction,
  onCriteriaChange,
  onDirectionChange,
  onApply,
  onReset,
  loading,
}: PlaylistSortProps) {
  return (
    <div className="playlist-control-card">
      <p className="eyebrow">Sort Songs</p>
      <div className="playlist-inline-form">
        <select
          value={criteria}
          onChange={(event) =>
            onCriteriaChange(event.target.value as 'title' | 'artist' | 'duration')
          }
          disabled={loading}
        >
          <option value="title">Title</option>
          <option value="artist">Artist</option>
          <option value="duration">Duration</option>
        </select>
        <select
          value={direction}
          onChange={(event) => onDirectionChange(event.target.value as 'asc' | 'desc')}
          disabled={loading}
        >
          <option value="asc">Ascending</option>
          <option value="desc">Descending</option>
        </select>
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
          Apply sort
        </button>
        <button type="button" className="secondary-button" onClick={onReset} disabled={loading}>
          Reset
        </button>
      </div>
    </div>
  )
}
