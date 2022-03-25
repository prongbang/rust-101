mod generic_function;
mod generic_struct;

fn main() {
    println!("Closures or Annonymous Functions or Lambdas");

    println!("Closures function sum single line");
    let sum = |x: i32, y: i32| x + y;
    println!("{}", sum(1, 2));

    println!("Closures function sum multiple line");
    let sum = |x: i32, y: i32| {
        // multiple statements
        x + y
    };
    println!("{}", sum(1, 2));

    println!("Closures parametor with function or higher order function sum inline function");
    let sum = |x: i32, y: i32| x + y;
    println!("{}", calculate(1, 2, sum));

    println!("Closures parametor with function or higher order function sum generic defined type function");
    let sum = |x: i32, y: i32| x + y;
    println!("{}", calculate_defined(1, 2, sum));

    println!("Closures parametor with function or higher order function sum where define type function");
    let sum = |x: i32, y: i32| x + y;
    println!("{}", calculate_where(1, 2, sum));

    println!("Generic function");
    generic_function::run_generic();
}

fn calculate(x: i32, y: i32, f: fn(i32, i32) -> i32) -> i32 {
    f(x, y)
}

fn calculate_defined<F: Fn(i32, i32) -> i32>(x: i32, y: i32, f: F) -> i32 {
    f(x, y)
}

fn calculate_where<F>(x: i32, y: i32, f: F) -> i32
    where
        F: Fn(i32, i32) -> i32
{
    f(x, y)
}

