# Web Playback Queue and Playlist Playback

## Scope

The web client now manages a playback queue for the current listening session.

This queue is used to:

- move to the previous song
- move to the next song
- keep the current song visible inside `Now Playing`
- start a full playlist from the playlist screens

## Queue behavior

The queue exists only in the web client session and is built from songs already returned
by the Rust server.

Two main queue sources are supported:

- the song list view, where the queue is based on the currently loaded server songs
- a playlist, where the queue is based on the songs that belong to that playlist

The currently playing song remains inside the queue view and is highlighted instead of being removed.
This makes the queue read naturally:

- previous songs appear above
- the current song appears highlighted
- next songs appear below

## Previous and next controls

`Now Playing` now supports:

- previous song
- play / pause current buffered song
- stop playback
- next song

If the current song ends and the queue still has another song, the frontend starts the next song automatically.
If there is no next song, the playback state is cleared.

## Playlist playback

Playlist playback is available from:

- the play button on playlist cards
- the main play button in playlist detail

When a playlist starts:

- its songs become the active queue
- the first song begins playback
- the user is taken to `Now Playing`

## UI cleanup

Non-functional clickable controls were removed so the interface only exposes actions that actually work.

Examples:

- unused top bar buttons were removed
- unused player-side action buttons were removed
- decorative playlist action buttons without behavior were removed
