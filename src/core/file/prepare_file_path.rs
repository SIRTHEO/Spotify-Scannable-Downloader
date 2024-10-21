use std::{error::Error, fs, path::Path};

pub fn prepare_file_path(
    content_type: &str,
    content_id: &str,
) -> Result<String, Box<dyn Error>> {
    let file_name = format!("{}.svg", content_id);
    let folder = format!("codes/{}", content_type);

    if !Path::new(&folder).exists() {
        fs::create_dir_all(&folder)?;
    }

    Ok(format!("{}/{}", folder, file_name))
}