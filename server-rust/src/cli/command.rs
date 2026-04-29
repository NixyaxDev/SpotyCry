#[derive(Debug)]
pub enum AdminCommand {
    Help,
    List,
    Add { path: String },
    Delete { song_id: String },
    Active { song_id: Option<String> },
    Exit,
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
