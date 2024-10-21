use xmltree::{Element, XMLNode};

pub fn is_logo_rect(elem: &Element) -> bool {
    elem.children.iter().any(|grandchild| {
        if let XMLNode::Element(ref grand_elem) = grandchild {
            if grand_elem.name == "path" {
                if let Some(d) = grand_elem.attributes.get("d") {
                    return d.contains("A30,30");
                }
            }
        }
        false
    })
}