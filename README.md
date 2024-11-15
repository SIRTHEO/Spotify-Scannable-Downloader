# Spotify Scannable Downloader

A tool to download Spotify scannable codes for sharing songs, albums, or playlists easily.

## How It Works

1. Provide a Spotify content URL.
2. (Optional) Specify a custom color for the scannable code.
3. The scannable code is downloaded and saved on your computer.

## Example Usage

Run the program with the following parameters:

```bash
./spotify-scannable-downloader -u "https://open.spotify.com/track/xyz" -c "blue"
cargo run --release -- -u "https://open.spotify.com/track/3Rc5bO1LQjlVPalGDwTsMa?si=69076081249245e0" -c "white"
```
- `-u`: Spotify content URL (required).
- `-c`: Code color (optional, default: black).
