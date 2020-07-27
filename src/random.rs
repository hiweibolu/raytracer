extern crate rand;

pub fn random_double() -> f64 {
    rand::random()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
