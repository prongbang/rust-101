fn main() {
    println!("Struct");

    let srt = Struct {
        a: "Devไปวันๆ".to_string(),
        b: 2,
    };
    println!("srt.a = {}", srt.a);
    println!("srt.b = {}", srt.b);
}

struct Struct {
    a: String,
    b: i32,
}