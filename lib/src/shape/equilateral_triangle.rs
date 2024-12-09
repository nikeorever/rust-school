use std::ops::{Div, Mul};
use crate::shape::area::Area;

pub struct EquilateralTriangle {
    edge_len: f64,
}

impl EquilateralTriangle {
    pub fn new(edge_len: f64) -> Self {
        Self { edge_len }
    }
}

#[cfg(feature = "area")]
impl Area for EquilateralTriangle {
    fn area(&self) -> f64 {
        3.0f64.sqrt().div(4.0f64).mul(self.edge_len.powi(2))
    }
}