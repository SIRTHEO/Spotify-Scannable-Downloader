use xmltree::Element;

pub fn is_background_rect(rect: &Element, viewbox_parts: &[&str]) -> bool {
    if viewbox_parts.len() == 4 {
        let viewbox_width: f64 = viewbox_parts[2].parse().unwrap_or(0.0);
        let viewbox_height: f64 = viewbox_parts[3].parse().unwrap_or(0.0);

        if let (Some(width), Some(height)) = (rect.attributes.get("width"), rect.attributes.get("height")) {
            let rect_width: f64 = width.trim().parse().unwrap_or(0.0);
            let rect_height: f64 = height.trim().parse().unwrap_or(0.0);

            return rect_width == viewbox_width && rect_height == viewbox_height;
        }
    }

    false
}
