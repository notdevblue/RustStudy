// https://blog.naver.com/kgs_safety/221928059157
// 변환법 블로그

use std::io::{self, stdin};

fn main() {
    loop {
        let mut buffer = String::new();
        
        println!("Input 1 for fahrenheit, 2 for celcius.");
        
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");
        
        let parsed: i32 = match buffer.trim().parse() {
            Ok(parsed_result) => parsed_result,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };
        
        let result = match parsed {
            1 => fahrenheit_to_celcius(),
            2 => celcius_to_fahrenheit(),
            _ => {
                println!("Invalid input");
                continue;
            }
        };

        println!("Converted result is: {result}");
        break;
    }

}

fn fahrenheit_to_celcius() -> f32 {
    (value_input() - 32) as f32 / 1.8
}

fn celcius_to_fahrenheit() -> f32 {
    (value_input() as f32 * 1.8) + 32.0
}

fn value_input() -> i32 {
    loop {
        let mut buffer = String::new();

        println!("Input value.");

        stdin().read_line(&mut buffer).expect("Failed to read line");

        let result: i32 = match buffer.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };

        return result;
    }
}
