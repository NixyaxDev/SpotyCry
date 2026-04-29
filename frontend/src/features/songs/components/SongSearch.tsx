import { useEffect, useState } from 'react'
import { MagnifyingGlassIcon } from '../../../shared/icons'

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
    // Se espera un pequeño debounce para no disparar una solicitud por cada tecla.
    const timeoutId = window.setTimeout(() => {
      onSearchChange(value)
    }, 300)

    return () => window.clearTimeout(timeoutId)
  }, [value, onSearchChange])

  return (
    <div className="songs-search">
      <MagnifyingGlassIcon />
      <select
        value={criteria}
        onChange={(event) =>
        onCriteriaChange(
          event.target.value as 'title' | 'artist' | 'album' | 'genre',
        )
        }
        aria-label="Buscar canciones por criterio"
      >
        <option value="title">Título</option>
        <option value="artist">Artista</option>
        <option value="album">Álbum</option>
        <option value="genre">Género</option>
      </select>
      <input
        value={value}
        onChange={(event) => setValue(event.target.value)}
        placeholder={`Buscar canciones por ${labelForCriteria(criteria).toLowerCase()}...`}
        aria-label={`Buscar canciones por ${labelForCriteria(criteria).toLowerCase()}`}
      />
      {loading && <span className="songs-search-status">Buscando...</span>}
    </div>
  )
}

function labelForCriteria(criteria: SongSearchProps['criteria']) {
  switch (criteria) {
    case 'title':
      return 'Título'
    case 'artist':
      return 'Artista'
    case 'album':
      return 'Álbum'
    case 'genre':
      return 'Género'
  }
}
