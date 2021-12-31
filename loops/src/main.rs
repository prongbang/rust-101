fn main() {
    println!("Loop: while");
    let mut x = 1;
    while x <= 3 {
        println!("x = {}", x);
        x += 1;
    }

    println!("Loop: loop");
    let mut x = 1;
    loop {
        println!("x = {}", x);
        x += 1;
        if x > 3 {
            break;
        }
    }

    println!("Loop: loop with break label");
    'outer: loop {
        println!("entering the outer loop");
        'inner: loop {
            println!("entering the inner loop");
            break 'outer;
        }
        println!("this point will not be reached");
    }

    println!("Loop: loop with continue and break label");
    let mut x = 0;
    'outer1: loop {
        x += 1;
        if x == 3 {
            break;
        }
        'inner2: loop {
            x += 1;
            if x == 6 {
                continue 'inner2;
            }
            println!("x = {}", x);
            if x == 5 {
                break 'outer1;
            }
        }
        println!("this point will not be reached");
    }

    println!("Loop: for 0..3 -> 0,1,2");
    for x in 0..3 {
        println!("x = {}", x);
    }
    println!("Loop: for 0..=3 -> 0,1,2,3");
    for x in 0..=3 {
        println!("x = {}", x);
    }

    println!("Loop: for numbers array");
    let numbers = [1, 2, 3, 4, 5];
    for x in numbers.iter() {
        println!("x = {}", x);
    }
    println!("Loop: for array [1,2,3]");
    for x in [1,2,3].iter() {
        println!("x = {}", x);
    }
    println!("Loop: for x.0, x.1 arrays [(1,2),(3,4),(5,6)]");
    for x in [(1,2),(3,4),(5,6)].iter() {
        println!("x.0 = {}", x.0);
        println!("x.1 = {}", x.1);
    }
    println!("Loop: for (x,y) arrays [(1,2),(3,4),(5,6)]");
    for (x, y) in [(1,2),(3,4),(5,6)].iter() {
        println!("x = {}, y = {}", x, y);
    }
    
}
