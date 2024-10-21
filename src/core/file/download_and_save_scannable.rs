use std::error::Error;

use reqwest::Client;

use crate::core::svg::process_svg::process_svg;

pub async fn download_and_save_scannable(
    url: &str,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(format!("Errore nel download: {}", response.status()).into());
    }

    let bytes = response.bytes().await?;

    process_svg(bytes, file_path)?;

    Ok(())
}