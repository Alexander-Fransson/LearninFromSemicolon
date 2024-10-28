#![allow(dead_code)]

pub fn match_examples() {
    
    let current_int = 3;

    match &current_int {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other")
    }

    let output = match current_int.to_string().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("Error: {}", e);
            0
        }
    };

    println!("output: {}", output);

    enum Direction {
        Up,
        Down,
        Left,
        Right
    }

    let current_direction = Direction::Left;

    match current_direction {
        Direction::Up => println!("Up"),
        Direction::Down => println!("Down"),
        Direction::Left => println!("Left"),
        Direction::Right => println!("Right")
    }

    enum RgbColor {
        Red = 0xFF0000,
        Green = 0x00FF00,
        Blue = 0x0000FF
    }

    let green = RgbColor::Green;

    match green {
        RgbColor::Red => println!("Red is {}", RgbColor::Red as i32),
        RgbColor::Green => println!("Green is {}", RgbColor::Green as i32),
        RgbColor::Blue => println!("Blue is {}", RgbColor::Blue as i32)
    }
}