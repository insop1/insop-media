use serde_json::json;

pub struct Metadata<'a> {
    player: &'a str,
    status: &'a str,
    artist: &'a str,
    title: &'a str,
    album: &'a str,
    art_url: &'a str,
}

impl<'a> Metadata<'a> {
    pub fn new(
        player: &'a str, 
        status: &'a str,
        artist: &'a str, 
        title: &'a str, 
        album: &'a str,
        art_url: &'a str, 
    ) -> Self { Self { player, status, artist, title, album, art_url } }

    // Will add config for these two soon
    fn text(&self) -> String {
        format!("{} - {}", self.artist, self.title)
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