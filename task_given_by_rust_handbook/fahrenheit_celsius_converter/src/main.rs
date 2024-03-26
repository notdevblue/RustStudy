// https://blog.naver.com/kgs_safety/221928059157
// 변환법 블로그

use std::io;

fn main() {
    loop {
        let mut buffer = String::new();
        
        println!("Input 1 for fahrenheit, 2 for celcius.");
        
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");
        
        let parsed: i32 = match buffer.parse() {
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

        println!("parsed result: {result}");
        break;
    }

}

fn fahrenheit_to_celcius() -> i32 {
    0
}

fn celcius_to_fahrenheit() -> i32 {
    0
}
