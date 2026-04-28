use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;

use tokio::sync::watch;

use crate::cli::command::{parse_command, AdminCommand};
use crate::songs::{SongLibrary, SongLibraryError};

pub fn start_admin_cli(
    song_library: Arc<Mutex<SongLibrary>>,
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
                    if execute_command(command, &song_library, &shutdown_sender) {
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
    shutdown_sender: &watch::Sender<bool>,
) -> bool {
    match command {
        AdminCommand::Help => print_help(),
        AdminCommand::List => list_songs(song_library),
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
