use serde_json::json;
use crate::config::Config;

pub struct Metadata<'a> {
    player: &'a str,
    status: &'a str,
    artist: &'a str,
    title: &'a str,
    album: &'a str,
    art_url: &'a str,
    config: &'a Config,
}

const ALLOWED_DYNAMIC: [&str; 5] = ["player", "artist", "title", "album", "art_url"];
impl<'a> Metadata<'a> {
    pub fn new(
        player: &'a str, 
        status: &'a str,
        artist: &'a str, 
        title: &'a str, 
        album: &'a str,
        art_url: &'a str, 
        config: &'a Config,
    ) -> Self { Self { player, status, artist, title, album, art_url, config } }

    // Will add config for these two soon
    fn text(&self) -> String {
        let dynamic: &Vec<String> = &self.config.dynamic;
        // wraps words with curly braces. Ex: {artist}
        let temp: Vec<String> = dynamic.iter()
            .filter(|x| ALLOWED_DYNAMIC.contains(&x.as_str()))
            .map(|x| {
                match x.as_str() {
                    "artist" => self.artist.to_string(),
                    "title" => self.title.to_string(),
                    "album" => self.album.to_string(),
                    "player" => self.player.to_string(),
                    "art_url" => self.art_url.to_string(),
                    _ => "error".to_string()
                }
            })
            .collect();

        temp.join(" - ")
    }
    fn tooltip(&self) -> String {
        format!("{}\n{}\n{}", self.artist, self.title, self.album)
    }
    pub fn waybar_print(&self, ) {
        // the glyph will be configurable eventually
        let text = match self.status {
            "Playing" => format!("{} {}", "󰏤", self.text()),
            "Paused" => format!("{} <i>{}</i>", "󰐊",self.text()),
            "Stopped" => "".to_string(),
            _ => String::new()
        };

        let output = json!({
            "text": text,
            "tooltip":  self.tooltip(),
            "class": self.status.to_lowercase(),
            "alt": self.player,
        });

        println!("{}", output);
    }
    #[allow(unused)]
    pub fn string(&self) -> String {
        format!("{}\n{}\n{}\n{}\n{}\n{}\n", 
            self.player, 
            self.status, 
            self.artist, 
            self.title, 
            self.album, 
            self.art_url
        )
    }
}

pub fn waybar_print_blank() {
    let output = json!({
        "text": "",
        "tooltip": "",
        "class": "",
        "alt": "",
    });

    println!("{}", output);
}
