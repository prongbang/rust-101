fn main() {
    println!("Struct");

    let srt = Struct {
        a: "Devไปวันๆ".to_string(),
        b: 2,
    };
    println!("srt.a = {}", srt.a);
    println!("srt.b = {}", srt.b);

    println!("Struct with new");
    let srt = Struct::new("Devไปวันๆ".to_string(), 2);
    println!("srt.a = {}", srt.a);
    println!("srt.b = {}", srt.b);

    println!("Struct with new and get a, b with method");
    let srt = Struct::new("Devไปวันๆ".to_string(), 2);
    println!("srt.a = {}", srt.get_a());
    println!("srt.b = {}", srt.get_b());
}

struct Struct {
    a: String,
    b: i32,
}

// Implement Struct
impl Struct {
    // method level module Struct
    pub fn new(a: String, b: i32) -> Self {
        Struct {
            a: a,
            b: b,
        }
    }

    // method for Struct
    pub fn get_a(&self) -> &str {
        &self.a  // Convert String to &str
    }

    // method for Struct
    pub fn get_b(&self) -> i32 {
        self.b
    }
}