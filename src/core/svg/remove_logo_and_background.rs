use std::error::Error;

use xmltree::{Element, XMLNode};

use super::{is_background_rect::is_background_rect, is_logo_rect::is_logo_rect};

pub fn remove_logo_and_background(root: &mut Element) -> Result<bool, Box<dyn Error>> {
    let mut removed = false;

    let viewbox = root.attributes.get("viewBox").unwrap_or(&"0 0 0 0".to_string()).clone();
    let viewbox_parts: Vec<&str> = viewbox.split_whitespace().collect();

    root.children.retain(|child| {
        if let XMLNode::Element(ref elem) = child {
            if elem.name == "g" && is_logo_rect(elem) {
                removed = true;
                return false;
            }

            if elem.name == "rect" && is_background_rect(elem, &viewbox_parts) {
                removed = true;
                return false;
            }
        }
        true
    });

    Ok(removed)
}
