pub fn build_scannable_url(code_color: &str, spotify_uri: &str) -> String {
    format!(
        "https://scannables.scdn.co/uri/plain/svg/00000/{}/{}/{}",
        code_color.to_lowercase(),
        640,  // Dimensione predefinita se il campo size non è presente
        spotify_uri
    )
}