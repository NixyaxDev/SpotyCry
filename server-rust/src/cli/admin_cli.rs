use std::fs;
use std::io::{self, Write};
use std::path::Path;
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
        println!("CLI de administración lista. Escribe 'help' para ver los comandos.");

        loop {
            print!("> ");

            if let Err(error) = io::stdout().flush() {
                eprintln!("Error al vaciar stdout: {}", error);
            }

            let mut input = String::new();

            match io::stdin().read_line(&mut input) {
                Ok(0) => {
                    println!("CLI finalizada.");
                    let _ = shutdown_sender.send(true);
                    break;
                }
                Ok(_) => {}
                Err(error) => {
                    eprintln!("Error al leer el comando: {}", error);
                    continue;
                }
            }

            match parse_command(&input) {
                Ok(command) => {
                    if execute_command(command, &song_library, &playlist_library, &shutdown_sender)
                    {
                        break;
                    }
                }
                Err(error) => eprintln!("Error: {}", error),
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
        AdminCommand::Playlist(command) => {
            execute_playlist_command(command, song_library, playlist_library)
        }
        AdminCommand::Active { song_id } => match song_id {
            Some(song_id) => set_active_song(song_library, &song_id),
            None => list_active_songs(song_library),
        },
        AdminCommand::Add { path } => add_song(song_library, &path),
        AdminCommand::AddDir { path } => add_song_directory(song_library, &path),
        AdminCommand::Delete { song_id } => delete_song(song_library, &song_id),
        AdminCommand::Exit => {
            println!("Cerrando el CLI de administración y el servidor...");
            let _ = shutdown_sender.send(true);
            return true;
        }
    }

    false
}

fn print_help() {
    println!("Comandos disponibles:");
    println!("  help                 Muestra los comandos disponibles");
    println!("  list                 Lista todas las canciones");
    println!("  search               Busca canciones por título, artista, álbum o género");
    println!("  playlist list        Lista todas las playlists");
    println!("  playlist create <name>");
    println!("                       Crea una playlist en la memoria del servidor");
    println!("  playlist songs <playlist-id>");
    println!("                       Lista las canciones que pertenecen a una playlist");
    println!("  playlist add-song <playlist-id> <song-id>");
    println!("                       Agrega una canción a una playlist");
    println!("  playlist remove-song <playlist-id> <song-id>");
    println!("                       Quita una canción de una playlist");
    println!("  playlist filter <playlist-id> <title|artist|genre> <value>");
    println!("                       Filtra canciones dentro de una playlist");
    println!("  playlist sort <playlist-id> <title|artist|duration> <asc|desc>");
    println!("                       Ordena canciones dentro de una playlist");
    println!("  playlist summary <playlist-id>");
    println!("                       Muestra estadísticas de una playlist");
    println!("  add <file-path>      Agrega una canción desde un archivo local");
    println!("  add-dir <folder-path>");
    println!(
        "                       Agrega todas las canciones soportadas desde una carpeta local"
    );
    println!("  delete <song-id>     Elimina una canción");
    println!("  active               Muestra las canciones activas");
    println!("  active <song-id>     Marca una canción como activa");
    println!("  exit                 Detiene el CLI y el servidor");
}

fn list_songs(song_library: &Arc<Mutex<SongLibrary>>) {
    match song_library.lock() {
        Ok(library) => {
            if library.songs().is_empty() {
                println!("No hay canciones cargadas.");
                return;
            }

            for song in library.songs() {
                println!("- {} | {}", song.id, format_song_summary(song));
            }
        }
        Err(_) => eprintln!("Error: No se pudo acceder a la biblioteca de canciones."),
    }
}

fn list_active_songs(song_library: &Arc<Mutex<SongLibrary>>) {
    match song_library.lock() {
        Ok(library) => {
            let active_songs = library.active_songs();

            if active_songs.is_empty() {
                println!("No hay canciones activas en este momento.");
                return;
            }

            for song in active_songs {
                println!("- {} | {}", song.id, format_song_summary(song));
            }
        }
        Err(_) => eprintln!("Error: No se pudo acceder a la biblioteca de canciones."),
    }
}

fn add_song(song_library: &Arc<Mutex<SongLibrary>>, path: &str) {
    match song_library.lock() {
        Ok(mut library) => match library.add_song(path) {
            Ok(song) => println!("Canción agregada: {} ({})", song.title, song.id),
            Err(error) => eprintln!("Error: {}", error),
        },
        Err(_) => eprintln!("Error: No se pudo acceder a la biblioteca de canciones."),
    }
}

fn add_song_directory(song_library: &Arc<Mutex<SongLibrary>>, path: &str) {
    let directory = Path::new(path);

    if !directory.exists() {
        eprintln!("Error: La carpeta no existe");
        return;
    }

    if !directory.is_dir() {
        eprintln!("Error: La ruta no corresponde a una carpeta");
        return;
    }

    let mut candidates = match fs::read_dir(directory) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|entry_path| is_supported_audio_file(entry_path))
            .collect::<Vec<_>>(),
        Err(_) => {
            eprintln!("Error: No se pudo leer el contenido de la carpeta");
            return;
        }
    };

    candidates.sort();

    if candidates.is_empty() {
        println!("No se encontraron archivos de audio soportados (.mp3/.wav) en esa carpeta.");
        return;
    }

    match song_library.lock() {
        Ok(mut library) => {
            let mut added = 0usize;
            let mut skipped = 0usize;

            for candidate in candidates {
                let display_path = candidate.to_string_lossy().to_string();

                match library.add_song(&display_path) {
                    Ok(song) => {
                        println!("Canción agregada: {} ({})", song.title, song.id);
                        added += 1;
                    }
                    Err(error) => {
                        eprintln!("Omitida {}: {}", display_path, error);
                        skipped += 1;
                    }
                }
            }

            println!(
                "Importación de carpeta finalizada. Agregadas: {} | Omitidas: {}",
                added, skipped
            );
        }
        Err(_) => eprintln!("Error: No se pudo acceder a la biblioteca de canciones."),
    }
}

fn is_supported_audio_file(path: &Path) -> bool {
    path.is_file()
        && path
            .extension()
            .and_then(|extension| extension.to_str())
            .map(|extension| {
                let normalized = extension.to_ascii_lowercase();
                normalized == "mp3" || normalized == "wav"
            })
            .unwrap_or(false)
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
        } => filter_playlist(
            song_library,
            playlist_library,
            &playlist_id,
            &criteria,
            &value,
        ),
        PlaylistCommand::Sort {
            playlist_id,
            criteria,
            direction,
        } => sort_playlist(
            song_library,
            playlist_library,
            &playlist_id,
            &criteria,
            &direction,
        ),
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
                println!("Todavía no se han creado playlists.");
                return;
            }

            for playlist in playlists {
                println!(
                    "- {} | {} | {} canciones",
                    playlist.id,
                    playlist.name,
                    playlist.song_ids.len()
                );
            }
        }
        Err(_) => eprintln!("Error: No se pudo acceder a la biblioteca de playlists."),
    }
}

fn create_playlist(playlist_library: &Arc<Mutex<PlaylistLibrary>>, name: &str) {
    match playlist_library.lock() {
        Ok(mut library) => match library.create_playlist(name) {
            Ok(playlist) => println!("Playlist creada: {} ({})", playlist.name, playlist.id),
            Err(error) => eprintln!("Error: {}", playlist_library_error_message(&error)),
        },
        Err(_) => eprintln!("Error: No se pudo acceder a la biblioteca de playlists."),
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
                eprintln!("Error: Playlist no encontrada");
                return;
            }
        },
        Err(_) => {
            eprintln!("Error: No se pudo acceder a la biblioteca de playlists.");
            return;
        }
    };

    match song_library.lock() {
        Ok(library) => {
            let songs = library.song_summaries_by_ids(&playlist.song_ids);

            if songs.is_empty() {
                println!("La playlist '{}' no tiene canciones.", playlist.name);
                return;
            }

            println!("Canciones en la playlist '{}':", playlist.name);
            for song in songs {
                println!("- {} | {}", song.id, format_song_summary_from_search(&song));
            }
        }
        Err(_) => eprintln!("Error: No se pudo acceder a la biblioteca de canciones."),
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
            eprintln!("Error: Canción no encontrada");
            return;
        }
        Ok(_) => {}
        Err(_) => {
            eprintln!("Error: No se pudo acceder a la biblioteca de canciones.");
            return;
        }
    }

    match playlist_library.lock() {
        Ok(mut library) => match library.add_song_to_playlist(playlist_id, song_id) {
            Ok(playlist) => println!(
                "Canción agregada a la playlist: {} ahora tiene {} canción(es)",
                playlist.name,
                playlist.song_ids.len()
            ),
            Err(error) => eprintln!("Error: {}", playlist_library_error_message(&error)),
        },
        Err(_) => eprintln!("Error: No se pudo acceder a la biblioteca de playlists."),
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
                "Canción eliminada de la playlist: {} ahora tiene {} canción(es)",
                playlist.name,
                playlist.song_ids.len()
            ),
            Err(error) => eprintln!("Error: {}", playlist_library_error_message(&error)),
        },
        Err(_) => eprintln!("Error: No se pudo acceder a la biblioteca de playlists."),
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
            eprintln!("Error: Criterio de filtro no soportado");
            return;
        }
        Err(_) => {
            eprintln!("Error: No se pudo interpretar el criterio de filtro");
            return;
        }
    };

    let playlist = match playlist_library.lock() {
        Ok(library) => match library.find_playlist(playlist_id) {
            Some(playlist) => playlist,
            None => {
                eprintln!("Error: Playlist no encontrada");
                return;
            }
        },
        Err(_) => {
            eprintln!("Error: No se pudo acceder a la biblioteca de playlists.");
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
                println!(
                    "No se encontraron canciones en la playlist '{}'.",
                    playlist.name
                );
                return;
            }

            println!("Canciones filtradas en la playlist '{}':", playlist.name);
            for song in songs {
                println!("- {} | {}", song.id, format_song_summary_from_search(&song));
            }
        }
        Err(_) => eprintln!("Error: No se pudo acceder a la biblioteca de canciones."),
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
            eprintln!("Error: Criterio de ordenamiento no soportado");
            return;
        }
        Err(_) => {
            eprintln!("Error: No se pudo interpretar el criterio de ordenamiento");
            return;
        }
    };

    let sort_direction = match parse_sort_direction(direction) {
        Ok(direction) => direction,
        Err(PlaylistOperationError::InvalidSortDirection) => {
            eprintln!("Error: La dirección de ordenamiento debe ser asc o desc");
            return;
        }
        Err(_) => {
            eprintln!("Error: No se pudo interpretar la dirección de ordenamiento");
            return;
        }
    };

    let playlist = match playlist_library.lock() {
        Ok(library) => match library.find_playlist(playlist_id) {
            Some(playlist) => playlist,
            None => {
                eprintln!("Error: Playlist no encontrada");
                return;
            }
        },
        Err(_) => {
            eprintln!("Error: No se pudo acceder a la biblioteca de playlists.");
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
                println!(
                    "La playlist '{}' no tiene canciones para ordenar.",
                    playlist.name
                );
                return;
            }

            println!("Canciones ordenadas en la playlist '{}':", playlist.name);
            for song in songs {
                println!("- {} | {}", song.id, format_song_summary_from_search(&song));
            }
        }
        Err(_) => eprintln!("Error: No se pudo acceder a la biblioteca de canciones."),
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
                eprintln!("Error: Playlist no encontrada");
                return;
            }
        },
        Err(_) => {
            eprintln!("Error: No se pudo acceder a la biblioteca de playlists.");
            return;
        }
    };

    match song_library.lock() {
        Ok(library) => {
            let summary =
                build_playlist_summary(&library.song_summaries_by_ids(&playlist.song_ids));
            println!("Resumen de la playlist '{}':", playlist.name);
            println!("- Canciones: {}", summary.song_count);
            println!("- Duración conocida: {}s", summary.total_duration_seconds);
            println!(
                "- Cantidad con duración desconocida: {}",
                summary.unknown_duration_count
            );
        }
        Err(_) => eprintln!("Error: No se pudo acceder a la biblioteca de canciones."),
    }
}

fn search_songs(song_library: &Arc<Mutex<SongLibrary>>) {
    let criteria = match prompt_cli_value("Criterio de búsqueda (title/artist/album/genre): ") {
        Ok(value) => value,
        Err(error) => {
            eprintln!("Error: {}", error);
            return;
        }
    };

    let normalized_criteria = criteria.trim().to_lowercase();

    if !matches!(
        normalized_criteria.as_str(),
        "title" | "artist" | "album" | "genre"
    ) {
        eprintln!("Error: Criterio no soportado. Usa title, artist, album o genre.");
        return;
    }

    let value = match prompt_cli_value("Valor de búsqueda: ") {
        Ok(value) => value,
        Err(error) => {
            eprintln!("Error: {}", error);
            return;
        }
    };

    match song_library.lock() {
        Ok(library) => match library.search_songs(&normalized_criteria, &value) {
            Some(results) if results.is_empty() => println!("No se encontraron canciones."),
            Some(results) => {
                println!(
                    "Se encontraron {} canción(es) usando '{}' como criterio de búsqueda:",
                    results.len(),
                    normalized_criteria
                );

                for song in results {
                    println!("- {} | {}", song.id, format_song_summary_from_search(&song));
                }
            }
            None => {
                eprintln!("Error: Criterio no soportado. Usa title, artist, album o genre.")
            }
        },
        Err(_) => eprintln!("Error: No se pudo acceder a la biblioteca de canciones."),
    }
}

fn delete_song(song_library: &Arc<Mutex<SongLibrary>>, song_id: &str) {
    match song_library.lock() {
        Ok(mut library) => match library.delete_song(song_id) {
            Ok(song) => println!("Canción eliminada: {}", song.id),
            Err(error) => eprintln!("Error: {}", song_library_error_message(&error)),
        },
        Err(_) => eprintln!("Error: No se pudo acceder a la biblioteca de canciones."),
    }
}

fn set_active_song(song_library: &Arc<Mutex<SongLibrary>>, song_id: &str) {
    match song_library.lock() {
        Ok(mut library) => match library.set_active_song(song_id) {
            Ok(song) => println!("Canción activa establecida: {} ({})", song.title, song.id),
            Err(error) => eprintln!("Error: {}", song_library_error_message(&error)),
        },
        Err(_) => eprintln!("Error: No se pudo acceder a la biblioteca de canciones."),
    }
}

fn song_library_error_message(error: &SongLibraryError) -> &'static str {
    match error {
        SongLibraryError::SongNotFound => "Canción no encontrada",
        SongLibraryError::SongInPlayback => {
            "No se puede eliminar la canción porque se está reproduciendo actualmente"
        }
    }
}

fn playlist_library_error_message(error: &PlaylistLibraryError) -> &'static str {
    match error {
        PlaylistLibraryError::InvalidName => "El nombre de la playlist no puede estar vacío",
        PlaylistLibraryError::AlreadyExists => "Ya existe una playlist con ese nombre",
        PlaylistLibraryError::PlaylistNotFound => "Playlist no encontrada",
        PlaylistLibraryError::SongAlreadyInPlaylist => {
            "La canción ya está en la playlist seleccionada"
        }
        PlaylistLibraryError::SongNotInPlaylist => {
            "La canción no pertenece a la playlist seleccionada"
        }
    }
}

fn format_song_summary(song: &crate::songs::Song) -> String {
    let artist = song.artist.as_deref().unwrap_or("Artista desconocido");
    let album = song.album.as_deref().unwrap_or("Álbum desconocido");
    let genre = song.genre.as_deref().unwrap_or("Género desconocido");
    let duration = song
        .duration
        .map(|seconds| format!("{}s", seconds))
        .unwrap_or_else(|| "Duración desconocida".to_string());

    format!(
        "{} | artista: {} | álbum: {} | género: {} | duración: {} | ruta: {}",
        song.title, artist, album, genre, duration, song.file_path
    )
}

fn format_song_summary_from_search(song: &SongSummary) -> String {
    let artist = song.artist.as_deref().unwrap_or("Artista desconocido");
    let album = song.album.as_deref().unwrap_or("Álbum desconocido");
    let genre = song.genre.as_deref().unwrap_or("Género desconocido");
    let duration = song
        .duration
        .map(|seconds| format!("{}s", seconds))
        .unwrap_or_else(|| "Duración desconocida".to_string());

    format!(
        "{} | artista: {} | álbum: {} | género: {} | duración: {}",
        song.title, artist, album, genre, duration
    )
}

fn prompt_cli_value(prompt: &str) -> Result<String, String> {
    print!("{}", prompt);
    io::stdout()
        .flush()
        .map_err(|error| format!("No se pudo vaciar stdout: {}", error))?;

    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .map_err(|error| format!("No se pudo leer la entrada: {}", error))?;

    Ok(value.trim().to_string())
}
