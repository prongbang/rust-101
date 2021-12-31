use std::collections::HashMap;

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

    println!("HashMap mutable insert");
    let mut hm: HashMap<i32, String> = HashMap::new();
    hm.insert(1, "one".to_string());
    println!("hm = {:?}", hm);

    println!("HashMap get by key");
    let one = hm.get(&1);
    println!("one = {:?}", one);

    println!("HashMap get value from Option with match");
    let one_value = match one {
        Some(x) => x,
        None => "None"
    };
    println!("one_value = {:?}", one_value);

    println!("HashMap get value from Option with unwrap or enum");
    let one_value = one.unwrap();
    println!("one_value = {:?}", one_value);
}
