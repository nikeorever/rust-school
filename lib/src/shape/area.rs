use std::ops::Add;

pub trait Area : Add<Output = Self> + Sized {
    fn area(&self) -> f64;
}