use std::thread;
use std::path::Path;
use crate::config::Config;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use crate::metadata::{self, Metadata};
use std::fs;
use std::os::unix::process::CommandExt;
// playerctl -p spotify --follow metadata --format '{{playerName}}|=|{{status}}|=|{{artist}}|=|{{title}}|=|{{album}}|=|{{mpris:artUrl}}'

pub fn run_event_loop(config: &Config, cache_dir: &Path) {
    let players = config.players.join(",");
    let mut command = Command::new("playerctl");
    command.args([
            "-p",
            &players,
            "--follow",
            "metadata",
            "--format",
            "{{playerName}}|=|{{status}}|=|{{artist}}|=|{{title}}|=|{{album}}|=|{{mpris:artUrl}}"
            ])
        .stdout(Stdio::piped())
        .stderr(Stdio::null());

    // kills child if parent exits abruptly
    unsafe {
        command.pre_exec(|| {
            libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGTERM);
            Ok(())
        });
    }

    let mut child = command.spawn().expect("playerctl not found");

    let stdout = child.stdout.take().expect("failed to take stdout");
    let reader = BufReader::new(stdout);

    // Creating a download thread for images
    // Also creates an optional tx
    // Checks if config.image is true
    let tx = if config.images {
        let (tx, rx) = std::sync::mpsc::channel::<String>();
        let thread_cache_dir = cache_dir.to_path_buf();
        thread::spawn(move || {
            for url in rx {
                download_url_image(&url, &thread_cache_dir);
            }
        });
        Some(tx)
    } else { 
        None 
    };

    let mut last_art_url: String = String::new();
    for line in reader.lines() {
        let Ok(line) = line else { continue };
        // playerctl emits a blank line when a player exits
        // this will clear up the waybar
        if line.is_empty() { 
            metadata::waybar_print_blank();
            last_art_url.clear();
            continue
        }

        let parts: Vec<&str> = line.split("|=|").collect();
        let [player, status, artist, title, album, art_url] = parts.as_slice() else { 
            eprintln!("Format wrong {line}");
            continue 
        };

        // Makes sure we're not downloading the same image
        // or attempting to download nothing
        if let Some(tx) = &tx {
            if last_art_url != *art_url && !art_url.is_empty() {
                let _ = tx.send(art_url.to_string());
                last_art_url.clear();
                last_art_url.push_str(art_url);
            }
        }
        
        let metadata = Metadata::new(player, status, artist, title, album, art_url);
        metadata.waybar_print();
    }
}

fn download_url_image(url: &str, cache_dir: &Path) {
    let image_path = cache_dir.join("current.jpg");
    let tmp_path = cache_dir.join("current.jpg.tmp");

    let Ok(response) = ureq::get(url).call() else {
        eprintln!("failed to download url image");
        return
    };

    let bytes = match response.into_body().read_to_vec() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("failed to read bytes {e}");
            return
        }
    };

    if let Err(e) = fs::write(&tmp_path, &bytes) {
        eprintln!("failed to write tmp file: {e}");
        return
    }

    if let Err(e) = fs::rename(&tmp_path, &image_path) {
        eprintln!("failed to swap tmp and image files: {e}");
        let _ = fs::remove_file(&tmp_path);
    }
}