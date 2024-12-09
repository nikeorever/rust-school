use lib::{Area, EquilateralTriangle};

fn main() {
    let edge_len = 4.0f64;
    let equilateral_triangle = EquilateralTriangle::new(edge_len);
    println!("The area of an equilateral triangle with edge length {edge_len} is {}", equilateral_triangle.area());
}
