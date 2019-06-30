pub trait Translate {
    fn translate(&self, dx: f64, dy: f64) -> Self;
}

impl<T: Translate + Clone> Translate for Vec<T> {
    fn translate(&self, dx: f64, dy: f64) -> Vec<T> {
        self.iter()
            .cloned()
            .map(|item| item.translate(dx, dy))
            .collect()
    }
}

pub trait Scale {
    fn scale(&self, scale: f64) -> Self;
}

impl<T: Scale + Clone> Scale for Vec<T> {
    fn scale(&self, scale: f64) -> Vec<T> {
        self.iter().cloned().map(|item| item.scale(scale)).collect()
    }
}
