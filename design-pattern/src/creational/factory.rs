pub trait Shape {
    fn draw(&self);
}

pub enum ShapeType {
    Rectangle,
    Circle,
}

pub struct Rectangle {}

impl Shape for Rectangle {
    fn draw(&self) {
        println!("\tdraw a rectangle");
    }
}

pub struct Circle {}

impl Shape for Circle {
    fn draw(&self) {
        println!("\tdraw a circle");
    }
}

pub struct ShapeFactory;

impl ShapeFactory {
    pub fn new(t: &ShapeType) -> Box<dyn Shape> {
        match t {
            ShapeType::Rectangle => Box::new(Rectangle {}),
            ShapeType::Circle => Box::new(Circle {}),
        }
    }
}