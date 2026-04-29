use crate::songs::SongSummary;

#[derive(Debug, Clone)]
pub struct PlaylistSummary {
    pub song_count: usize,
    pub total_duration_seconds: u64,
    pub unknown_duration_count: usize,
}

pub fn build_playlist_summary(songs: &[SongSummary]) -> PlaylistSummary {
    songs.iter().fold(
        PlaylistSummary {
            song_count: 0,
            total_duration_seconds: 0,
            unknown_duration_count: 0,
        },
        |accumulator, song| PlaylistSummary {
            song_count: accumulator.song_count + 1,
            total_duration_seconds: accumulator.total_duration_seconds + song.duration.unwrap_or(0),
            unknown_duration_count: accumulator.unknown_duration_count
                + usize::from(song.duration.is_none()),
        },
    )
}
