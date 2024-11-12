use std::error::Error;

use xmltree::Element;

pub fn calculate_viewbox(root: &Element) -> Result<(f64, f64, f64, f64), Box<dyn Error>> {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for child in &root.children {
        if let Some(element) = child.as_element() {
            if element.name == "rect" {
                if let (Some(x), Some(y), Some(width), Some(height)) = (
                    element.attributes.get("x"),
                    element.attributes.get("y"),
                    element.attributes.get("width"),
                    element.attributes.get("height"),
                ) {
                    let x: f64 = x.parse()?;
                    let y: f64 = y.parse()?;
                    let width: f64 = width.parse()?;
                    let height: f64 = height.parse()?;

                    min_x = min_x.min(x);
                    min_y = min_y.min(y);
                    max_x = max_x.max(x + width);
                    max_y = max_y.max(y + height);
                }
            }
        }
    }

    let width = max_x - min_x;

    let height = max_y - min_y;

    Ok((width, height, min_x, min_y))
}