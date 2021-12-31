fn main() {
    println!("Function");

    println!("Function without parameters");
    fn_without_parameters();

    println!("Function with parameters");
    fn_with_parameters("Devไปวันๆ".to_string());

    println!("Function without parameters return value");
    let h = fn_without_parameters_return();
    println!("{}", h);

    println!("Function with parameters return value");
    let h = fn_with_parameters_return("Devไปวันๆ".to_string());
    println!("{}", h);

    println!("Function with parameters return value and mutable");
    let h = fn_with_mutable_parameters_return("Devไปวันๆ".to_string());
    println!("{}", h);

}

fn fn_without_parameters() {
    println!("Hello: Function");
}

fn fn_with_parameters(name: String) {
    println!("Hello: {}", name);
}

fn fn_without_parameters_return() -> String {
    return "Hello: Function Return".to_string();
}

fn fn_with_parameters_return(name: String) -> String {
    return format!("Hello: {} Function Return", name).to_string();
}

fn fn_with_mutable_parameters_return(mut name: String) -> String {
    name += " Hello";
    return format!("{} Function Return", name).to_string();
}