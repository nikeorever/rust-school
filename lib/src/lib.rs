mod shape;

pub use crate::shape::area::Area;
pub use crate::shape::equilateral_triangle::EquilateralTriangle;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(EquilateralTriangle::new(4.0f64).area(), 6.928203230275509f64);
    }
}
