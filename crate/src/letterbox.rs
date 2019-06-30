use std::f64;

pub struct Letterbox {
    pub left: f64,
    pub top: f64,
    pub scale: f64,
}

impl Letterbox {
    pub fn new(ideal_dimensions: (u32, u32), actual_dimensions: (u32, u32)) -> Letterbox {
        let (ideal_width, ideal_height) = ideal_dimensions;
        let (ideal_width, ideal_height) = (f64::from(ideal_width), f64::from(ideal_height));
        let (actual_width, actual_height) = actual_dimensions;
        let (actual_width, actual_height) = (f64::from(actual_width), f64::from(actual_height));

        let ideal_aspect = Letterbox::aspect((ideal_width, ideal_height));
        let actual_aspect = Letterbox::aspect((actual_width, actual_height));

        if actual_aspect > ideal_aspect {
            let scale = actual_height / ideal_height;
            let scaled_width = ideal_width * scale;
            let left = (actual_width - scaled_width) / 2.0;

            Letterbox {
                left,
                top: 0.0,
                scale,
            }
        } else {
            let scale = actual_width / ideal_width;
            let scaled_height = ideal_height * scale;
            let top = (actual_height - scaled_height) / 2.0;

            Letterbox {
                left: 0.0,
                top,
                scale,
            }
        }
    }

    fn aspect(dimensions: (f64, f64)) -> f64 {
        let (width, height) = dimensions;

        width / height
    }
}
