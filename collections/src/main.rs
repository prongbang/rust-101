fn main() {
    println!("Collection");

    println!("Vec mutable with push");
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v = {:?}", v);

    println!("Vec mutable with marcro defined and pop");
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let x = v.pop();
    println!("v = {:?}", v);
    println!("x = {:?}", x);

    println!("Vec immutable with marcro defined");
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("v = {:?}", v);

}
