use xmltree::Element;



pub fn reposition_elements(root: &mut Element, min_x: f64, min_y: f64) {
    for child in &mut root.children {
        if let Some(element) = child.as_mut_element() {
            if element.name == "rect" {
                if let Some(x) = element.attributes.get_mut("x") {
                    let x_value: f64 = x.parse().unwrap_or(0.0);
                    *x = (x_value - min_x).to_string();
                }

                if let Some(y) = element.attributes.get_mut("y") {
                    let y_value: f64 = y.parse().unwrap_or(0.0);
                    *y = (y_value - min_y).to_string();
                }
            }
        }
    }
}