pub mod core;
pub mod models;

use clap::Parser;

use core::file::download_and_save_scannable::download_and_save_scannable;
use core::file::prepare_file_path::prepare_file_path;
use core::uri::build_scannable_url::build_scannable_url;
use core::uri::extract_spotify_info::extract_spotify_info;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Opt {
    #[arg(short = 'u', long)]
    spotify_url: String,

    #[arg(short = 'c', long)]
    code_color: Option<String>, 
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::parse();

    let code_color = opt.code_color.unwrap_or_else(|| "black".to_string());

    let spotify_info = extract_spotify_info(&opt.spotify_url)?;
    let scannable_url = build_scannable_url(&code_color, &spotify_info.spotify_uri);

    let file_path = prepare_file_path(
        &spotify_info.content_type,
        &spotify_info.content_id,
    )?;

    download_and_save_scannable(&scannable_url, &file_path).await?;

    Ok(())
}