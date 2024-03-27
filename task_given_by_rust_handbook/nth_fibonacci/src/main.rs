use std::io;

fn main() {
    loop {
        let mut buf = String::new();

        println!("Input you number");

        io::stdin().read_line(&mut buf).expect("Failed to read line");

        let n: i64 = match buf.trim().parse() {
            Ok(res) => res,
            Err(_) => continue
        };

        println!(
            "{}{} fibonacci number is {}",
            n,
            match n {
                1 => "st",
                2 => "nd",
                3 => "rd",
                _ => "th"
            },
            fib(n)
        );
        break;
    }
}

fn fib(n: i64) -> i64 {
    if n <= 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
