# insop-media

## About

A playerctl wrapper daemon for waybar with more features than the MPRIS module. Instead of using ignored-players, we explicitly list our players into "players" into the config.json file.

## Install

Requires `rust` and `playerctl`.

```bash
git clone https://github.com/Insop1/insop-media
cd insop-media
cargo install --path .
```

## Config

Insop-Media requires you to have a config file in `~/.config/insop-media`. You can overwrite default commands using special-commands. Default commands are `playerctl -p <players> <command>`.
Note: dynamic and scroll-text are currently WIP.

```json
{
    "players": ["spotatui", "spotify"],
    "scroll-text": false,
    "images": true,  
    "dynamic": [
        "artist", "title"
    ],
    "special-commands": {
        "play-pause": {
            "spotify": "playerctl -p spotify play-pause"
        },
        "next": {

        },
        "previous": {

        }
    }
}
```