fn main() {
    println!("String stack: slice &str");
    let s = "hello world";
    println!("s = {}", s);

    println!("String heap: std::string::String");
    let s = String::from("hello world");
    println!("s = {}", s);

    println!("String stack to String heap");
    let s = "hello world".to_string();
    println!("s = {}", s);
}