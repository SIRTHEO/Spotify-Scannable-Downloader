use std::error::Error;
use url::Url;
use crate::models::spotify_info::SpotifyInfo;

pub fn extract_spotify_info(spotify_url: &str) -> Result<SpotifyInfo, Box<dyn Error>> {
    let parsed_url = Url::parse(spotify_url)?;

    match parsed_url.host_str() {
        Some(host) if host.contains("spotify.com") => (),
        _ => return Err("L'URL fornito non è un URL di Spotify valido.".into()),
    }

    let segments: Vec<&str> = parsed_url
        .path_segments()
        .map(|c| c.collect())
        .unwrap_or_default();

    if segments.len() < 2 {
        return Err("L'URL di Spotify non contiene abbastanza informazioni.".into());
    }

    // TODO -> Cambiare metodo di segmentazione:
    // Errore generato con: https://open.spotify.com/intl-it/track/3Rc5bO1LQjlVPalGDwTsMa?si=69076081249245e0
    let content_type = segments[0].to_string();
    let content_id = segments[1].to_string();
    let spotify_uri = format!("spotify:{}:{}", content_type, content_id);

    Ok(SpotifyInfo {
        spotify_uri,
        content_type,
        content_id,
    })
}