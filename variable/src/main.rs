
fn main() {
    println!("Constant Variable");
    const PI:f64 = 3.14;
    println!("PI = {}", PI);

    println!("Mutable Variable");
    // immutable variable
    let x: i32;
    let y: i32 = 2;
    x = 1;
    println!("x = {}", x);
    println!("y = {}", y);

    let x = 10;
    println!("x = {}", x);
    let (x, y) = (1, 2);
    println!("x = {}", x);
    println!("y = {}", y);

    // mutable variable
    println!("Immutable Variable");
    let mut z: i32 = 4;
    let mut a: i32;
    z = 5;
    a = 6;
    a = 7;
    println!("z = {}", z);
    println!("a = {}", a);

    println!("Tuple Variable");
    let x = (1, 2, 3.14);
    println!("x = {:?}", x);
    println!("x.0 = {}", x.0);
    println!("x.1 = {}", x.1);
    println!("x.2 = {}", x.2);
    let x: (i32, i32, f64) = (1, 2, 3.14);
    println!("x = {:?}", x);
    println!("x.0 = {}", x.0);
    println!("x.1 = {}", x.1);
    println!("x.2 = {}", x.2);

    println!("Array Variable");
    let x:[i32;5] = [1, 2, 3, 4, 5];
    println!("x = {:?}", x);
    println!("x[0] = {}", x[0]);
    println!("x[1] = {}", x[1]);
    println!("x[2] = {}", x[2]);
    println!("x[3] = {}", x[3]);
    println!("x[4] = {}", x[4]);
    let a = [1, 2, 3, 4, 5];
    println!("a = {:?}", a);
    println!("a[0] = {}", a[0]);
    println!("a[1] = {}", a[1]);
    println!("a[2] = {}", a[2]);
    println!("a[3] = {}", a[3]);
    println!("a[4] = {}", a[4]);
    let x = [1;5];
    println!("x = {:?}", x);        // [1, 1, 1, 1, 1]

}
