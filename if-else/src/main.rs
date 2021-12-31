fn main() {
    println!("#1 IF ELSE: Standard");
    let score = 90;
    let grade: &str;
    if score >= 80 {
        grade = "A";
    } else if score >= 60 {
        grade = "B";
    } else if score >= 40 {
        grade = "C";
    } else {
        grade = "D";
    }
    println!("score = {}, grade = {}", score, grade);

    println!("#2 IF ELSE: Ternary operator");
    let score = 40;
    let grade = if score >= 80 {
        "A"
    } else if score >= 60 {
        "B"
    } else if score >= 40 {
        "C"
    } else {
        "D"
    };
    println!("score = {}, grade = {}", score, grade);

    let is_passed = score >= 60;
    println!("is_passed = {}", is_passed);
    let is_success = if is_passed {"success"} else {"failure" };
    println!("is_success = {}", is_success);
}
