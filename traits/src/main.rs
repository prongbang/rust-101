// main using module
use traits::{
    aes::Aes,
    cryptography::Cryptography,
};

fn main() {
    println!("Traits or interface");

    println!("Implement Cryptography trait to Aes struct");
    let aes = Aes::new("1234567890123456".to_string());
    println!("Aes encryption using function in Cryptography trait");
    println!("{}", aes.encrypt("Hello World"));
    println!("Aes decryption using function in Cryptography trait");
    println!("{}", aes.decrypt("XYXZABC"));
}
