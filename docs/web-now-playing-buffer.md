# Web Playback Buffer and Now Playing

## Decision

The web client keeps a **local buffer only for the current song being played**.

This buffer is built from the audio chunks sent by the Rust server over WebSocket.
Once all chunks for the selected song are received, the frontend reconstructs a `Blob`,
creates an object URL, and assigns it to the browser audio element.

## Why this satisfies the requirement

This approach satisfies the requirement:

- only the current song is buffered locally
- the full song is available in browser memory while it remains selected
- the user can seek forward and backward as many times as needed for that same song
- the client does not download the whole catalog

Because the browser audio element receives a full local `Blob` URL for the current song,
the user can use the native seek bar repeatedly without re-requesting audio from the server.

## Important playback behavior

The playback controls are now split into two different actions:

- `pause`: keeps the current song in the local browser buffer
- `stop`: releases the current playback state and clears the buffered song

This distinction is important because pausing should not destroy the local buffer.
If pausing cleared the audio state, the user would lose the ability to continue seeking
within the current song.

## Now Playing section

The `Now Playing` screen is connected to the real playback state and now shows:

- current song information
- whether the song is buffering, playing, or buffered locally and ready
- a play/pause control for the buffered song
- a stop control to release the current playback
- an `Up Next` list that can start another server song

The bottom player remains the real HTML audio control surface for:

- play
- pause
- timeline seek
- repeated forward/backward seeking on the current song

## Resource cleanup

The local buffer is cleared when:

- the user explicitly stops playback
- a different song starts playing
- the player state is reset

This keeps the implementation simple and aligned with the scope of the project.
