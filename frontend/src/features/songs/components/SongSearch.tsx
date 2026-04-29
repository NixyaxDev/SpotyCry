import { useEffect, useState } from 'react'

type SongSearchProps = {
  initialValue?: string
  onSearchChange: (value: string) => void
  loading: boolean
}

export function SongSearch({
  initialValue = '',
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
      <input
        value={value}
        onChange={(event) => setValue(event.target.value)}
        placeholder="Search songs by title..."
        aria-label="Search songs by title"
      />
      {loading && <span className="songs-search-status">Searching...</span>}
    </div>
  )
}
