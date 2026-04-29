#[derive(Debug)]
pub enum AdminCommand {
    Help,
    List,
    Search,
    Playlist(PlaylistCommand),
    Add { path: String },
    Delete { song_id: String },
    Active { song_id: Option<String> },
    Exit,
}

#[derive(Debug)]
pub enum PlaylistCommand {
    List,
    Create { name: String },
    Songs { playlist_id: String },
    AddSong { playlist_id: String, song_id: String },
    RemoveSong { playlist_id: String, song_id: String },
    Filter {
        playlist_id: String,
        criteria: String,
        value: String,
    },
    Sort {
        playlist_id: String,
        criteria: String,
        direction: String,
    },
    Summary { playlist_id: String },
}

pub fn parse_command(input: &str) -> Result<AdminCommand, String> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err("No command entered. Type 'help' to see the available commands.".to_string());
    }

    let mut parts = trimmed.split_whitespace();
    let command = parts.next().ok_or_else(|| {
        "No command entered. Type 'help' to see the available commands.".to_string()
    })?;

    match command.to_lowercase().as_str() {
        "help" => Ok(AdminCommand::Help),
        "list" => Ok(AdminCommand::List),
        "search" => {
            if parts.next().is_some() {
                Err("Usage: search".to_string())
            } else {
                Ok(AdminCommand::Search)
            }
        }
        "playlist" => parse_playlist_command(parts.collect()),
        "active" => {
            let song_id = parts.next().map(|value| value.to_string());

            if parts.next().is_some() {
                Err("Usage: active or active <song-id>".to_string())
            } else {
                Ok(AdminCommand::Active { song_id })
            }
        }
        "exit" => Ok(AdminCommand::Exit),
        "add" => {
            let path = parts.collect::<Vec<_>>().join(" ");

            if path.is_empty() {
                Err("Usage: add <local-file-path>".to_string())
            } else {
                Ok(AdminCommand::Add { path })
            }
        }
        "delete" => {
            let song_id = parts
                .next()
                .ok_or_else(|| "Usage: delete <song-id>".to_string())?;

            if parts.next().is_some() {
                Err("Usage: delete <song-id>".to_string())
            } else {
                Ok(AdminCommand::Delete {
                    song_id: song_id.to_string(),
                })
            }
        }
        _ => Err(format!(
            "Unknown command: '{}'. Type 'help' to see the available commands.",
            command
        )),
    }
}

fn parse_playlist_command(parts: Vec<&str>) -> Result<AdminCommand, String> {
    let subcommand = parts
        .first()
        .ok_or_else(|| {
            "Usage: playlist <list|create|songs|add-song|remove-song|filter|sort|summary>"
                .to_string()
        })?
        .to_lowercase();

    let remaining = &parts[1..];

    let command = match subcommand.as_str() {
        "list" if remaining.is_empty() => PlaylistCommand::List,
        "create" => {
            let name = remaining.join(" ");

            if name.trim().is_empty() {
                return Err("Usage: playlist create <name>".to_string());
            }

            PlaylistCommand::Create { name }
        }
        "songs" => {
            let playlist_id = remaining
                .first()
                .ok_or_else(|| "Usage: playlist songs <playlist-id>".to_string())?;

            if remaining.len() != 1 {
                return Err("Usage: playlist songs <playlist-id>".to_string());
            }

            PlaylistCommand::Songs {
                playlist_id: (*playlist_id).to_string(),
            }
        }
        "add-song" => {
            if remaining.len() != 2 {
                return Err("Usage: playlist add-song <playlist-id> <song-id>".to_string());
            }

            PlaylistCommand::AddSong {
                playlist_id: remaining[0].to_string(),
                song_id: remaining[1].to_string(),
            }
        }
        "remove-song" => {
            if remaining.len() != 2 {
                return Err("Usage: playlist remove-song <playlist-id> <song-id>".to_string());
            }

            PlaylistCommand::RemoveSong {
                playlist_id: remaining[0].to_string(),
                song_id: remaining[1].to_string(),
            }
        }
        "filter" => {
            if remaining.len() < 3 {
                return Err(
                    "Usage: playlist filter <playlist-id> <title|artist|genre> <value>"
                        .to_string(),
                );
            }

            PlaylistCommand::Filter {
                playlist_id: remaining[0].to_string(),
                criteria: remaining[1].to_string(),
                value: remaining[2..].join(" "),
            }
        }
        "sort" => {
            if remaining.len() != 3 {
                return Err(
                    "Usage: playlist sort <playlist-id> <title|artist|duration> <asc|desc>"
                        .to_string(),
                );
            }

            PlaylistCommand::Sort {
                playlist_id: remaining[0].to_string(),
                criteria: remaining[1].to_string(),
                direction: remaining[2].to_string(),
            }
        }
        "summary" => {
            let playlist_id = remaining
                .first()
                .ok_or_else(|| "Usage: playlist summary <playlist-id>".to_string())?;

            if remaining.len() != 1 {
                return Err("Usage: playlist summary <playlist-id>".to_string());
            }

            PlaylistCommand::Summary {
                playlist_id: (*playlist_id).to_string(),
            }
        }
        _ => {
            return Err(
                "Usage: playlist <list|create|songs|add-song|remove-song|filter|sort|summary>"
                    .to_string(),
            )
        }
    };

    Ok(AdminCommand::Playlist(command))
}
