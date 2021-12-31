fn main() {
    println!("Enum");

    let r = Color::Red;
    println!("Check enum and print");
    match r {
        Color::Red => {
            // Multiple statements
            println!("Red");
        },
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::Unknown => println!("Unknown"),
    }
    
    println!("Check enum and set to variable");
    let g = Color::Green;
    let color = match g {
        Color::Red => "Red",
        Color::Green => "Green",
        Color::Blue => "Blue",
        _ => "Unknown",
    };
    println!("{}", color);
    
    println!("Check enum and set to variable else case");
    let b = Color::Blue;
    let color = match b {
        Color::Red => "Red",
        Color::Green => "Green",
        _ => "Blue",
    };
    println!("{}", color);

    println!("Check enum and send custom ok parameter from function");
    let result = get_color_name(Color::Red);
    match result {
        ColorResult::Ok(color) => println!("{}", color),
        ColorResult::Err(error) => println!("{}", error),
    }

    println!("Check enum and send custom err parameter from function");
    let result = get_color_name(Color::Unknown);
    match result {
        ColorResult::Ok(color) => println!("{}", color),
        ColorResult::Err(error) => println!("{}", error),
    }
        
    println!("Check enum and send option some parameter from function");
    let result = get_color_name_option(Color::Red);
    match result {
        Option::Some(color) => println!("{}", color),
        Option::None => println!("None"),
    }
        
    println!("Check enum and send option none parameter from function");
    let result = get_color_name_option(Color::Unknown);
    match result {
        Option::Some(color) => println!("{}", color),
        Option::None => println!("None"),
    }
        
    println!("Check enum and send result ok parameter from function");
    let result = get_color_name_result(Color::Blue);
    match result {
        Result::Ok(color) => println!("{}", color),
        Result::Err(error) => println!("{}", error),
    }
        
    println!("Check enum and send result err parameter from function");
    let result = get_color_name_result(Color::Unknown);
    match result {
        Result::Ok(color) => println!("{}", color),
        Result::Err(error) => println!("{}", error),
    }
        
    // unwrap using guard pattern
    println!("Check enum and send result err parameter from function using unwrap guard pattern");
    let result = get_color_name_result(Color::Unknown);
    if result.is_err() {
        let error = result.unwrap_err();
        println!("{}", error);
    } else {
        let color = result.unwrap();
        println!("{}", color);
    }
        
    // unwrap using if let pattern
    println!("Check enum and send result err parameter from function using unwrap if let pattern");
    let result = get_color_name_result(Color::Blue);
    if let Ok(color) = result {
        println!("{}", color);
    } else {
        let error = result.unwrap_err();
        println!("{}", error);
    }
        
    // unwrap using match return pattern
    println!("Check enum and send result err parameter from function using unwrap match return pattern");
    let result = get_color_name_result(Color::Blue);
    let color = match result {
        Err(error) => {
            println!("{}", error);
            return;
        },
        Ok(color) => color,
    };
    println!("{}", color);

    // method unwrap not recommended
    println!("Check enum and send result ok parameter from function using unwrap");
    let result = get_color_name_result(Color::Red);
    let color = result.unwrap();
    println!("{}", color);
        
    println!("Check enum and send result err parameter from function using unwrap panic");
    let result = get_color_name_result(Color::Unknown);
    let color = result.unwrap();
    println!("{}", color);

}

fn get_color_name(color: Color) -> ColorResult {
    match color {
        Color::Red => ColorResult::Ok("Red".to_string()),
        Color::Green => ColorResult::Ok("Green".to_string()),
        Color::Blue => ColorResult::Ok("Blue".to_string()),
        _ => ColorResult::Err("Unknown color".to_string()),
    }
}

fn get_color_name_option(color: Color) -> Option<String> {
    match color {
        Color::Red => Option::Some("Red".to_string()),
        Color::Green => Option::Some("Green".to_string()),
        Color::Blue => Option::Some("Blue".to_string()),
        _ => Option::None,
    }
}

fn get_color_name_result(color: Color) -> Result<String, String> {
    match color {
        Color::Red => Result::Ok("Red".to_string()),
        Color::Green => Result::Ok("Green".to_string()),
        Color::Blue => Result::Ok("Blue".to_string()),
        _ => Result::Err("Unknown color".to_string()),
    }
}

enum Color {
    Red,
    Green,
    Blue,
    Unknown,
}

enum ColorResult {
    Ok(String),
    Err(String),
}