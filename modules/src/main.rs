// main using module
// package-name::module-file::struct-name
use modules::bar::Bar;

fn main() {
    println!("Module using struct Foo");
    // package-name::module-file::struct-name
    let foo = modules::foo::Foo::new( "Devไปวันๆ".to_string());
    let bar = foo.get_bar();
    println!("bar.foo{}", bar.get_foo());

    println!("Module using struct Bar");
    let bar = Bar::new("Devไปวันๆ".to_string());
    println!("bar.foo = {}", bar.get_foo());
}
