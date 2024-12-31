use std::ops::{Add, Sub};

pub trait Area : Add<Output = Self> + Sub<Output = Self> + Sized {
    fn area(&self) -> f64;
}