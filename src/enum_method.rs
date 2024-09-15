pub enum Shape {
    _Circle(f64),
    Square(f64),
    _Rectangle(f64, f64),
}
impl Shape {
    pub fn area(&self) -> f64 {
        let area = match self {
            Shape::_Circle(r) => 3.14 * r * r,
            Shape::Square(s) => s * s,
            Shape::_Rectangle(w, h) => w * h,
        };
        area
    }
}
