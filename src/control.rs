use crate::config::Config;
use std::process::Command;
use std::collections::HashMap;

fn run_control(
    config: &Config, 
    subcommand: &str, 
    value: Option<&str>, 
    special_command: 
    &HashMap<String, String>
) {
    if let Some(player) = get_priority_player(config) {
        if let Some(cmd) = special_command.get(&player) {
            let _ = Command::new("sh").args(["-c", &cmd]).status();
            return
        }
    }

    let players = config.players.join(",");
    let mut args = vec!["-p", &players, subcommand];
    if let Some(v) = value {
        args.push(v);
    } 
    
    let _ = Command::new("playerctl").args(&args).status();
}

fn get_priority_player(config: &Config) -> Option<String> {
    let output = Command::new("playerctl")
        .args(["-l"])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let active_players: Vec<&str> = stdout
        .lines()
        .map(|l| l.split('.').next().unwrap_or(l))
        .collect();

    config.players
        .iter()
        .find(|p| active_players.contains(&p.as_str()))
        .cloned()
}

pub fn toggle(config: &Config) {
    run_control(config, "play-pause", None, &config.special_commands.play_pause);
}
pub fn next(config: &Config) {
    run_control(config, "next", None, &config.special_commands.next);
}
pub fn previous(config: &Config) {
    run_control(config, "previous", None,  &config.special_commands.previous);
}