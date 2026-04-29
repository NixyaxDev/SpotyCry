import { useEffect, useState } from 'react'

type SongSearchProps = {
  criteria: 'title' | 'artist' | 'album' | 'genre'
  initialValue?: string
  onCriteriaChange: (criteria: 'title' | 'artist' | 'album' | 'genre') => void
  onSearchChange: (value: string) => void
  loading: boolean
}

export function SongSearch({
  criteria,
  initialValue = '',
  onCriteriaChange,
  onSearchChange,
  loading,
}: SongSearchProps) {
  const [value, setValue] = useState(initialValue)

  useEffect(() => {
    const timeoutId = window.setTimeout(() => {
      onSearchChange(value)
    }, 300)

    return () => window.clearTimeout(timeoutId)
  }, [value, onSearchChange])

  return (
    <div className="songs-search">
      <span className="material-symbols-outlined">search</span>
      <select
        value={criteria}
        onChange={(event) =>
          onCriteriaChange(
            event.target.value as 'title' | 'artist' | 'album' | 'genre',
          )
        }
        aria-label="Search songs by criteria"
      >
        <option value="title">Title</option>
        <option value="artist">Artist</option>
        <option value="album">Album</option>
        <option value="genre">Genre</option>
      </select>
      <input
        value={value}
        onChange={(event) => setValue(event.target.value)}
        placeholder={`Search songs by ${criteria}...`}
        aria-label={`Search songs by ${criteria}`}
      />
      {loading && <span className="songs-search-status">Searching...</span>}
    </div>
  )
}
