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

    fn lookup(&self, key: &str) -> Option<&str> {
        match key {
            "artist" => Some(self.artist),
            "title" => Some(self.title),
            "album" => Some(self.album),
            "player" => Some(self.player),
            "art_url" => Some(self.art_url),
            _ => None,
        }
    }
    fn text(&self) -> String {
        let dynamic: &Vec<String> = &self.config.dynamic;
        let temp: Vec<&str> = dynamic.iter()
            .filter_map(|x| self.lookup(x))
            .collect();

        temp.join(" - ")
    }
    // WIP
    fn tooltip(&self) -> String {
        format!("{}\n{}\n{}", self.artist, self.title, self.album)
    }
    pub fn waybar_print(&self) {
        // the glyph will be configurable eventually
        let text = match self.status {
            "Playing" => format!("{} {}", "󰏤", self.text()),
            "Paused" => format!("{} <i>{}</i>", "󰐊",self.text()),
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
