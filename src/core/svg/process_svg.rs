use std::{error::Error, fs};

use xmltree::Element;

use super::{calculate_viewbox::calculate_viewbox, remove_logo_and_background::remove_logo_and_background, reposition_elements::reposition_elements};

pub fn process_svg(bytes: bytes::Bytes, file_path: &str) -> Result<(), Box<dyn Error>> {
    let svg_content = String::from_utf8(bytes.to_vec())?;
    let mut root = Element::parse(svg_content.as_bytes())?;

    remove_logo_and_background(&mut root)?;

    let (width, height, min_x, min_y) = calculate_viewbox(&root)?;
    let viewbox = format!("0 0 {} {}", width, height);
    root.attributes.insert("viewBox".to_string(), viewbox);
    root.attributes.insert("width".to_string(), width.to_string());
    root.attributes.insert("height".to_string(), height.to_string());

    reposition_elements(&mut root, min_x, min_y);

    let mut buffer = Vec::new();
    root.write(&mut buffer)?;
    let optimized_svg = String::from_utf8(buffer)?;

    fs::write(file_path, optimized_svg)?;
    Ok(())
}