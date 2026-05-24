mod metadata;
mod config;
mod control;
mod daemon;

use std::fs;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(name = "insop-media")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Run,
    Toggle,
    Next,
    Previous
}

fn main() {
    let args = Cli::parse();

    let home = PathBuf::from(std::env::var("HOME").expect("HOME not set"));
    let config_path = home.join(".config/insop-media/config.json");
    let cache_dir = home.join(".cache/insop-media");

    fs::create_dir_all(&cache_dir).expect("failed to create cache dir");
    let Some(config) = config::load_config(&config_path) else {
        eprintln!("couldn't load config");
        return
    };

    if config.players.is_empty() { return }
    match args.command {
        Commands::Run => daemon::run_event_loop(&config, &cache_dir),
        Commands::Toggle => control::toggle(&config),
        Commands::Next => control::next(&config),
        Commands::Previous => control::previous(&config),
    }
}