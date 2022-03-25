#[derive(Clone)]
pub struct Product {
    parts: Vec<String>,
}

impl Product {
    pub fn new() -> Product {
        Product { parts: Vec::new() }
    }

    pub fn list_parts(&self) {
        for v in &self.parts {
            println!("{}", v);
        }
    }
}

pub trait Builder {
    fn produce_part_a(&mut self);
    fn produce_part_b(&mut self);
    fn produce_part_c(&mut self);
    fn get_product(&mut self) -> Product;
}

pub struct ConcreteBuilder1 {
    pub product: Product,
}

impl ConcreteBuilder1 {
    pub fn new() -> ConcreteBuilder1 {
        ConcreteBuilder1 {
            product: Product::new(),
        }
    }
}

impl Builder for ConcreteBuilder1 {
    fn produce_part_a(&mut self) {
        self.product.parts.push("\tpart a1".to_string());
    }

    fn produce_part_b(&mut self) {
        self.product.parts.push("\tpart b1".to_string());
    }

    fn produce_part_c(&mut self) {
        self.product.parts.push("\tpart c1".to_string());
    }

    fn get_product(&mut self) -> Product {
        let p = self.product.clone();
        self.product = Product::new();
        return p;
    }
}

pub struct ConcreteBuilder2 {
    pub product: Product,
}

impl ConcreteBuilder2 {
    pub fn new() -> ConcreteBuilder2 {
        ConcreteBuilder2 {
            product: Product::new(),
        }
    }
}

impl Builder for ConcreteBuilder2 {
    fn produce_part_a(&mut self) {
        self.product.parts.push("\tpart a2".to_string());
    }

    fn produce_part_b(&mut self) {
        self.product.parts.push("\tpart b2".to_string());
    }

    fn produce_part_c(&mut self) {
        self.product.parts.push("\tpart c2".to_string());
    }

    fn get_product(&mut self) -> Product {
        let p = Product {
            parts: self.product.parts.clone(),
            ..self.product
        };
        self.product = Product::new();
        return p;
    }
}

// The Director is only responsible for executing the building
pub struct Director {
    pub builder: Box<dyn Builder>,
}

impl Director {
    pub fn new(builder: Box<dyn Builder>) -> Director {
        Director { builder }
    }

    pub fn construct(&mut self) {
        self.builder.produce_part_a();
        self.builder.produce_part_b();
        self.builder.produce_part_c();
    }
}