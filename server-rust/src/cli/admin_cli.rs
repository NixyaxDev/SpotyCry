use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;

use tokio::sync::watch;

use crate::cli::command::{parse_command, AdminCommand, PlaylistCommand};
use crate::playlists::{
    build_playlist_summary, filter_playlist_songs, parse_filter_criteria, parse_sort_criteria,
    parse_sort_direction, sort_playlist_songs, PlaylistLibrary, PlaylistLibraryError,
    PlaylistOperationError,
};
use crate::songs::{SongLibrary, SongLibraryError, SongSummary};

pub fn start_admin_cli(
    song_library: Arc<Mutex<SongLibrary>>,
    playlist_library: Arc<Mutex<PlaylistLibrary>>,
    shutdown_sender: watch::Sender<bool>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        println!("🛠️  CLI de administración lista. Escribe 'help' para ver los comandos.");

        loop {
            print!("> ");

            if let Err(error) = io::stdout().flush() {
                eprintln!("✖ Error flushing stdout: {}", error);
            }

            let mut input = String::new();

            match io::stdin().read_line(&mut input) {
                Ok(0) => {
                    println!("👋 CLI finalizada.");
                    let _ = shutdown_sender.send(true);
                    break;
                }
                Ok(_) => {}
                Err(error) => {
                    eprintln!("✖ Error reading command: {}", error);
                    continue;
                }
            }

            match parse_command(&input) {
                Ok(command) => {
                    if execute_command(
                        command,
                        &song_library,
                        &playlist_library,
                        &shutdown_sender,
                    ) {
                        break;
                    }
                }
                Err(error) => eprintln!("✖ Error: {}", error),
            }
        }
    })
}

fn execute_command(
    command: AdminCommand,
    song_library: &Arc<Mutex<SongLibrary>>,
    playlist_library: &Arc<Mutex<PlaylistLibrary>>,
    shutdown_sender: &watch::Sender<bool>,
) -> bool {
    match command {
        AdminCommand::Help => print_help(),
        AdminCommand::List => list_songs(song_library),
        AdminCommand::Search => search_songs(song_library),
        AdminCommand::Playlist(command) => execute_playlist_command(
            command,
            song_library,
            playlist_library,
        ),
        AdminCommand::Active { song_id } => match song_id {
            Some(song_id) => set_active_song(song_library, &song_id),
            None => list_active_songs(song_library),
        },
        AdminCommand::Add { path } => add_song(song_library, &path),
        AdminCommand::Delete { song_id } => delete_song(song_library, &song_id),
        AdminCommand::Exit => {
            println!("👋 Closing admin CLI and server...");
            let _ = shutdown_sender.send(true);
            return true;
        }
    }

    false
}

fn print_help() {
    println!("Available commands:");
    println!("  help                 Show available commands");
    println!("  list                 List all songs");
    println!("  search               Search songs by title, artist, album or genre");
    println!("  playlist list        List all playlists");
    println!("  playlist create <name>");
    println!("                       Create a playlist in the server memory");
    println!("  playlist songs <playlist-id>");
    println!("                       List songs that belong to a playlist");
    println!("  playlist add-song <playlist-id> <song-id>");
    println!("                       Add a song to a playlist");
    println!("  playlist remove-song <playlist-id> <song-id>");
    println!("                       Remove a song from a playlist");
    println!("  playlist filter <playlist-id> <title|artist|genre> <value>");
    println!("                       Filter songs inside a playlist");
    println!("  playlist sort <playlist-id> <title|artist|duration> <asc|desc>");
    println!("                       Sort songs inside a playlist");
    println!("  playlist summary <playlist-id>");
    println!("                       Show playlist statistics");
    println!("  add <file-path>      Add a song from a local file");
    println!("  delete <song-id>     Delete a song");
    println!("  active               Show currently active songs");
    println!("  active <song-id>     Mark a song as active");
    println!("  exit                 Stop the CLI and the server");
}

fn list_songs(song_library: &Arc<Mutex<SongLibrary>>) {
    match song_library.lock() {
        Ok(library) => {
            if library.songs().is_empty() {
                println!("No songs loaded.");
                return;
            }

            for song in library.songs() {
                println!("- {} | {}", song.id, format_song_summary(song));
            }
        }
        Err(_) => eprintln!("✖ Error: Could not access the song library."),
    }
}

fn list_active_songs(song_library: &Arc<Mutex<SongLibrary>>) {
    match song_library.lock() {
        Ok(library) => {
            let active_songs = library.active_songs();

            if active_songs.is_empty() {
                println!("No active songs right now.");
                return;
            }

            for song in active_songs {
                println!("- {} | {}", song.id, format_song_summary(song));
            }
        }
        Err(_) => eprintln!("✖ Error: Could not access the song library."),
    }
}

fn add_song(song_library: &Arc<Mutex<SongLibrary>>, path: &str) {
    match song_library.lock() {
        Ok(mut library) => match library.add_song(path) {
            Ok(song) => println!("✔ Song added: {} ({})", song.title, song.id),
            Err(error) => eprintln!("✖ Error: {}", error),
        },
        Err(_) => eprintln!("✖ Error: Could not access the song library."),
    }
}

fn execute_playlist_command(
    command: PlaylistCommand,
    song_library: &Arc<Mutex<SongLibrary>>,
    playlist_library: &Arc<Mutex<PlaylistLibrary>>,
) {
    match command {
        PlaylistCommand::List => list_playlists(playlist_library),
        PlaylistCommand::Create { name } => create_playlist(playlist_library, &name),
        PlaylistCommand::Songs { playlist_id } => {
            list_playlist_songs(song_library, playlist_library, &playlist_id)
        }
        PlaylistCommand::AddSong {
            playlist_id,
            song_id,
        } => add_song_to_playlist(song_library, playlist_library, &playlist_id, &song_id),
        PlaylistCommand::RemoveSong {
            playlist_id,
            song_id,
        } => remove_song_from_playlist(playlist_library, &playlist_id, &song_id),
        PlaylistCommand::Filter {
            playlist_id,
            criteria,
            value,
        } => filter_playlist(song_library, playlist_library, &playlist_id, &criteria, &value),
        PlaylistCommand::Sort {
            playlist_id,
            criteria,
            direction,
        } => sort_playlist(song_library, playlist_library, &playlist_id, &criteria, &direction),
        PlaylistCommand::Summary { playlist_id } => {
            summarize_playlist(song_library, playlist_library, &playlist_id)
        }
    }
}

fn list_playlists(playlist_library: &Arc<Mutex<PlaylistLibrary>>) {
    match playlist_library.lock() {
        Ok(library) => {
            let playlists = library.playlists();

            if playlists.is_empty() {
                println!("No playlists created yet.");
                return;
            }

            for playlist in playlists {
                println!(
                    "- {} | {} | {} songs",
                    playlist.id,
                    playlist.name,
                    playlist.song_ids.len()
                );
            }
        }
        Err(_) => eprintln!("✖ Error: Could not access the playlist library."),
    }
}

fn create_playlist(playlist_library: &Arc<Mutex<PlaylistLibrary>>, name: &str) {
    match playlist_library.lock() {
        Ok(mut library) => match library.create_playlist(name) {
            Ok(playlist) => println!("✔ Playlist created: {} ({})", playlist.name, playlist.id),
            Err(error) => eprintln!("✖ Error: {}", playlist_library_error_message(&error)),
        },
        Err(_) => eprintln!("✖ Error: Could not access the playlist library."),
    }
}

fn list_playlist_songs(
    song_library: &Arc<Mutex<SongLibrary>>,
    playlist_library: &Arc<Mutex<PlaylistLibrary>>,
    playlist_id: &str,
) {
    let playlist = match playlist_library.lock() {
        Ok(library) => match library.find_playlist(playlist_id) {
            Some(playlist) => playlist,
            None => {
                eprintln!("✖ Error: Playlist not found");
                return;
            }
        },
        Err(_) => {
            eprintln!("✖ Error: Could not access the playlist library.");
            return;
        }
    };

    match song_library.lock() {
        Ok(library) => {
            let songs = library.song_summaries_by_ids(&playlist.song_ids);

            if songs.is_empty() {
                println!("Playlist '{}' has no songs.", playlist.name);
                return;
            }

            println!("Songs in playlist '{}':", playlist.name);
            for song in songs {
                println!("- {} | {}", song.id, format_song_summary_from_search(&song));
            }
        }
        Err(_) => eprintln!("✖ Error: Could not access the song library."),
    }
}

fn add_song_to_playlist(
    song_library: &Arc<Mutex<SongLibrary>>,
    playlist_library: &Arc<Mutex<PlaylistLibrary>>,
    playlist_id: &str,
    song_id: &str,
) {
    match song_library.lock() {
        Ok(library) if !library.has_song(song_id) => {
            eprintln!("✖ Error: Song not found");
            return;
        }
        Ok(_) => {}
        Err(_) => {
            eprintln!("✖ Error: Could not access the song library.");
            return;
        }
    }

    match playlist_library.lock() {
        Ok(mut library) => match library.add_song_to_playlist(playlist_id, song_id) {
            Ok(playlist) => println!(
                "✔ Song added to playlist: {} now has {} song(s)",
                playlist.name,
                playlist.song_ids.len()
            ),
            Err(error) => eprintln!("✖ Error: {}", playlist_library_error_message(&error)),
        },
        Err(_) => eprintln!("✖ Error: Could not access the playlist library."),
    }
}

fn remove_song_from_playlist(
    playlist_library: &Arc<Mutex<PlaylistLibrary>>,
    playlist_id: &str,
    song_id: &str,
) {
    match playlist_library.lock() {
        Ok(mut library) => match library.remove_song_from_playlist(playlist_id, song_id) {
            Ok(playlist) => println!(
                "✔ Song removed from playlist: {} now has {} song(s)",
                playlist.name,
                playlist.song_ids.len()
            ),
            Err(error) => eprintln!("✖ Error: {}", playlist_library_error_message(&error)),
        },
        Err(_) => eprintln!("✖ Error: Could not access the playlist library."),
    }
}

fn filter_playlist(
    song_library: &Arc<Mutex<SongLibrary>>,
    playlist_library: &Arc<Mutex<PlaylistLibrary>>,
    playlist_id: &str,
    criteria: &str,
    value: &str,
) {
    let filter_criteria = match parse_filter_criteria(criteria) {
        Ok(criteria) => criteria,
        Err(PlaylistOperationError::InvalidFilterCriteria) => {
            eprintln!("✖ Error: Unsupported filter criterion");
            return;
        }
        Err(_) => {
            eprintln!("✖ Error: Could not parse filter criterion");
            return;
        }
    };

    let playlist = match playlist_library.lock() {
        Ok(library) => match library.find_playlist(playlist_id) {
            Some(playlist) => playlist,
            None => {
                eprintln!("✖ Error: Playlist not found");
                return;
            }
        },
        Err(_) => {
            eprintln!("✖ Error: Could not access the playlist library.");
            return;
        }
    };

    match song_library.lock() {
        Ok(library) => {
            let songs = filter_playlist_songs(
                &library.song_summaries_by_ids(&playlist.song_ids),
                filter_criteria,
                value,
            );

            if songs.is_empty() {
                println!("No songs found in playlist '{}'.", playlist.name);
                return;
            }

            println!("Filtered songs in playlist '{}':", playlist.name);
            for song in songs {
                println!("- {} | {}", song.id, format_song_summary_from_search(&song));
            }
        }
        Err(_) => eprintln!("✖ Error: Could not access the song library."),
    }
}

fn sort_playlist(
    song_library: &Arc<Mutex<SongLibrary>>,
    playlist_library: &Arc<Mutex<PlaylistLibrary>>,
    playlist_id: &str,
    criteria: &str,
    direction: &str,
) {
    let sort_criteria = match parse_sort_criteria(criteria) {
        Ok(criteria) => criteria,
        Err(PlaylistOperationError::InvalidSortCriteria) => {
            eprintln!("✖ Error: Unsupported sort criterion");
            return;
        }
        Err(_) => {
            eprintln!("✖ Error: Could not parse sort criterion");
            return;
        }
    };

    let sort_direction = match parse_sort_direction(direction) {
        Ok(direction) => direction,
        Err(PlaylistOperationError::InvalidSortDirection) => {
            eprintln!("✖ Error: Sort direction must be asc or desc");
            return;
        }
        Err(_) => {
            eprintln!("✖ Error: Could not parse sort direction");
            return;
        }
    };

    let playlist = match playlist_library.lock() {
        Ok(library) => match library.find_playlist(playlist_id) {
            Some(playlist) => playlist,
            None => {
                eprintln!("✖ Error: Playlist not found");
                return;
            }
        },
        Err(_) => {
            eprintln!("✖ Error: Could not access the playlist library.");
            return;
        }
    };

    match song_library.lock() {
        Ok(library) => {
            let songs = sort_playlist_songs(
                &library.song_summaries_by_ids(&playlist.song_ids),
                sort_criteria,
                sort_direction,
            );

            if songs.is_empty() {
                println!("Playlist '{}' has no songs to sort.", playlist.name);
                return;
            }

            println!("Sorted songs in playlist '{}':", playlist.name);
            for song in songs {
                println!("- {} | {}", song.id, format_song_summary_from_search(&song));
            }
        }
        Err(_) => eprintln!("✖ Error: Could not access the song library."),
    }
}

fn summarize_playlist(
    song_library: &Arc<Mutex<SongLibrary>>,
    playlist_library: &Arc<Mutex<PlaylistLibrary>>,
    playlist_id: &str,
) {
    let playlist = match playlist_library.lock() {
        Ok(library) => match library.find_playlist(playlist_id) {
            Some(playlist) => playlist,
            None => {
                eprintln!("✖ Error: Playlist not found");
                return;
            }
        },
        Err(_) => {
            eprintln!("✖ Error: Could not access the playlist library.");
            return;
        }
    };

    match song_library.lock() {
        Ok(library) => {
            let summary = build_playlist_summary(&library.song_summaries_by_ids(&playlist.song_ids));
            println!("Playlist summary for '{}':", playlist.name);
            println!("- Songs: {}", summary.song_count);
            println!("- Known duration: {}s", summary.total_duration_seconds);
            println!("- Unknown duration count: {}", summary.unknown_duration_count);
        }
        Err(_) => eprintln!("✖ Error: Could not access the song library."),
    }
}

fn search_songs(song_library: &Arc<Mutex<SongLibrary>>) {
    let criteria = match prompt_cli_value("Search criterion (title/artist/album/genre): ") {
        Ok(value) => value,
        Err(error) => {
            eprintln!("✖ Error: {}", error);
            return;
        }
    };

    let normalized_criteria = criteria.trim().to_lowercase();

    if !matches!(
        normalized_criteria.as_str(),
        "title" | "artist" | "album" | "genre"
    ) {
        eprintln!("✖ Error: Unsupported criterion. Use title, artist, album or genre.");
        return;
    }

    let value = match prompt_cli_value("Search value: ") {
        Ok(value) => value,
        Err(error) => {
            eprintln!("✖ Error: {}", error);
            return;
        }
    };

    match song_library.lock() {
        Ok(library) => match library.search_songs(&normalized_criteria, &value) {
            Some(results) if results.is_empty() => println!("No songs found."),
            Some(results) => {
                println!(
                    "Found {} song(s) using '{}' as search criterion:",
                    results.len(),
                    normalized_criteria
                );

                for song in results {
                    println!("- {} | {}", song.id, format_song_summary_from_search(&song));
                }
            }
            None => {
                eprintln!("✖ Error: Unsupported criterion. Use title, artist, album or genre.")
            }
        },
        Err(_) => eprintln!("✖ Error: Could not access the song library."),
    }
}

fn delete_song(song_library: &Arc<Mutex<SongLibrary>>, song_id: &str) {
    match song_library.lock() {
        Ok(mut library) => match library.delete_song(song_id) {
            Ok(song) => println!("✔ Song removed: {}", song.id),
            Err(error) => eprintln!("✖ Error: {}", song_library_error_message(&error)),
        },
        Err(_) => eprintln!("✖ Error: Could not access the song library."),
    }
}

fn set_active_song(song_library: &Arc<Mutex<SongLibrary>>, song_id: &str) {
    match song_library.lock() {
        Ok(mut library) => match library.set_active_song(song_id) {
            Ok(song) => println!("✔ Active song set: {} ({})", song.title, song.id),
            Err(error) => eprintln!("✖ Error: {}", song_library_error_message(&error)),
        },
        Err(_) => eprintln!("✖ Error: Could not access the song library."),
    }
}

fn song_library_error_message(error: &SongLibraryError) -> &'static str {
    match error {
        SongLibraryError::SongNotFound => "Song not found",
        SongLibraryError::SongInPlayback => {
            "Cannot delete song because it is currently being played"
        }
    }
}

fn playlist_library_error_message(error: &PlaylistLibraryError) -> &'static str {
    match error {
        PlaylistLibraryError::InvalidName => "Playlist name cannot be empty",
        PlaylistLibraryError::AlreadyExists => "A playlist with that name already exists",
        PlaylistLibraryError::PlaylistNotFound => "Playlist not found",
        PlaylistLibraryError::SongAlreadyInPlaylist => {
            "Song is already in the selected playlist"
        }
        PlaylistLibraryError::SongNotInPlaylist => "Song is not in the selected playlist",
    }
}

fn format_song_summary(song: &crate::songs::Song) -> String {
    let artist = song.artist.as_deref().unwrap_or("Unknown artist");
    let album = song.album.as_deref().unwrap_or("Unknown album");
    let genre = song.genre.as_deref().unwrap_or("Unknown genre");
    let duration = song
        .duration
        .map(|seconds| format!("{}s", seconds))
        .unwrap_or_else(|| "Unknown duration".to_string());

    format!(
        "{} | artist: {} | album: {} | genre: {} | duration: {} | path: {}",
        song.title, artist, album, genre, duration, song.file_path
    )
}

fn format_song_summary_from_search(song: &SongSummary) -> String {
    let artist = song.artist.as_deref().unwrap_or("Unknown artist");
    let album = song.album.as_deref().unwrap_or("Unknown album");
    let genre = song.genre.as_deref().unwrap_or("Unknown genre");
    let duration = song
        .duration
        .map(|seconds| format!("{}s", seconds))
        .unwrap_or_else(|| "Unknown duration".to_string());

    format!(
        "{} | artist: {} | album: {} | genre: {} | duration: {}",
        song.title, artist, album, genre, duration
    )
}

fn prompt_cli_value(prompt: &str) -> Result<String, String> {
    print!("{}", prompt);
    io::stdout()
        .flush()
        .map_err(|error| format!("Could not flush stdout: {}", error))?;

    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .map_err(|error| format!("Could not read input: {}", error))?;

    Ok(value.trim().to_string())
}
