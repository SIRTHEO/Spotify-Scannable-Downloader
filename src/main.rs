use clap::Parser;
use reqwest::Error;
use std::fs;
use std::path::Path;
use url::Url;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opt {
    /// URL di Spotify (es. https://open.spotify.com/album/5Lxs0AM3WPdKzWxYhrYYgv?si=...)
    #[arg(short, long)]
    spotify_url: String,

    /// Formato dell'immagine (es. png, svg). Default: svg
    #[arg(short, long, default_value = "svg")]
    format: String,

    /// Colore di sfondo in esadecimale (es. ffffff per bianco). Default: ffffff
    #[arg(short = 'b', long, default_value = "ffffff")]
    background_color: String,

    /// Colore del codice (es. black, white). Default: black
    #[arg(short = 'c', long, default_value = "black")]
    code_color: String,

    /// Dimensione dell'immagine in pixel (es. 640). Default: 640
    #[arg(short, long, default_value_t = 640)]
    size: u32,

    /// Nome del file da salvare (senza estensione). Default: spotify_code
    #[arg(short, long, default_value = "spotify_code")]
    file_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Parsing degli argomenti della riga di comando
    let opt = Opt::parse();

    // Estrai il Spotify URI dall'URL fornito
    let spotify_uri = match extract_spotify_uri(&opt.spotify_url) {
        Ok(uri) => uri,
        Err(e) => {
            eprintln!("Errore: {}", e);
            std::process::exit(1);
        }
    };

    // Costruisci l'URL del codice scannabile
    let scannable_url = format!(
        "https://scannables.scdn.co/uri/plain/{}/{}/{}/{}/{}",
        opt.format.to_lowercase(),
        opt.background_color.to_lowercase(),
        opt.code_color.to_lowercase(),
        opt.size,
        spotify_uri
    );

    println!("URL del codice scannabile: {}", scannable_url);

    // Nome del file con estensione
    let file_name = format!("{}.{}", opt.file_name, opt.format);
    let folder = "codes";

    // Creare la cartella se non esiste
    if !Path::new(folder).exists() {
        fs::create_dir(folder).expect("Impossibile creare la cartella 'codes'");
        println!("Cartella 'codes' creata.");
    }

    let file_path = format!("{}/{}", folder, file_name);

    // Scaricare il contenuto dell'URL
    let response = reqwest::get(&scannable_url).await?;

    // Verificare che la richiesta abbia avuto successo
    if response.status().is_success() {
        let bytes = response.bytes().await?;

        fs::write(&file_path, &bytes).expect("Impossibile scrivere il file");

        println!("Codice scannabile scaricato con successo in '{}'", file_path);
    } else {
        eprintln!(
            "Errore nel download: {}",
            response.status()
        );
    }

    Ok(())
}

/// Estrae l'URI di Spotify dall'URL fornito.
/// Supporta album, playlist, artista e brano.
fn extract_spotify_uri(spotify_url: &str) -> Result<String, String> {
    // Parse l'URL
    let parsed_url = Url::parse(spotify_url).map_err(|e| format!("URL non valido: {}", e))?;

    // Verifica il dominio
    if !parsed_url.host_str().unwrap_or("").contains("spotify.com") {
        return Err("L'URL fornito non è un URL di Spotify valido.".to_string());
    }

    // Ottieni il path e dividilo in parti
    let segments: Vec<&str> = parsed_url.path_segments().map(|c| c.collect()).unwrap_or_else(Vec::new);

    if segments.len() < 2 {
        return Err("L'URL di Spotify non contiene abbastanza informazioni.".to_string());
    }

    // Primo segmento: tipo di contenuto (album, playlist, artist, track)
    let content_type = segments[0];

    // Secondo segmento: ID del contenuto
    let content_id = segments[1];

    // Costruisci l'URI di Spotify
    let spotify_uri = format!("spotify:{}:{}", content_type, content_id);

    Ok(spotify_uri)
}