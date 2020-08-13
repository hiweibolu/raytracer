#![allow(clippy::all)]

pub fn random_double() -> f64 {
    rand::random()
}

/*pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn random_int_range(min: i32, max: i32) -> i32 {
    (random_double_range(min as f64, max as f64)).floor() as i32
}*/
