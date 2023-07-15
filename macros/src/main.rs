// Macro rules
macro_rules! say_macro_rules {
    () => (println!("Macro Rules"));
}

macro_rules! multiply_macro_rules {
    ($expr:expr) => {
        println!("The value of the expression is: {}", $expr * 2);
    };
}

fn main() {
    say_macro_rules!();
    multiply_macro_rules!(3);
}
