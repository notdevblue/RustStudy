use std::io;

// https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
fn variables() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // let x = 5;
    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }

    // println!("The value of x is: {x}");

    // let spaces = "   ";
    // let spaces = spaces.len();
    // println!("The spaces: {spaces}");

    // let mut spaces = "   ";
    // spaces = spaces.len(); // expected &str found usize
}

// https://doc.rust-lang.org/book/ch03-02-data-types.html
fn datatypes() {
    // let _guess: u32 = "42".parse().expect("NaN");
    // // let guess: u32 = "42".parse() 랑
    // // let guess = "42".parse::<u32>() 도 가능

    // let _iteral = 0xff;
    // let _iteral = 0b0001;
    // let _iteral = 0o12;
    // let _iteral = 1_541;
    // let _iteral = b'A'; // 'A' 를 하면 'A' 가 저장되지만, b'A' 하면 'A' 의 ascii
    // // 123123i32 같이 해도 형 지정이 됨

    // let _floating_point = 1.54; // 1.54f64 / double
    // let _floating_point: f32 = 1.54; // 1.54f32 / float

    // println!("sum: {}", 5 + 15);
    // println!("sub: {}", 15 - 5);
    // println!("mul: {}", 2 * 2);
    // println!("div: {}", 10.0 / 3f64);
    // println!("div (truncated): {}", -10 / 3);
    // println!("remain: {}", 10 % 3);

    // let _contition = true;
    // let _contition = false;

    // if _contition {
    //     println!("it is true");
    // } else {
    //     println!("it is false");
    // }

    // let _c = 'z';
    // let _c = '와';
    // let _c = '♨';
    // // 4바이트 unicode scalar
    // // 일반적은 char 랑 조금 다름
    
    // let _tup = (154, 154.1, true);
    // let _tup: (&str, char, f64) = ("Hello", 'a', 154.1);
    // let _tup = (1, 5, 4);

    // let (x, y, z) = _tup; // 구조 분해 할당. js/ts 에 있던 그거
    // println!("x: {x}, y: {y}, z: {z}");
    
    // let x = _tup.0;
    // let y = _tup.1;
    // let z = _tup.2;
    // println!("x: {x}, y: {y}, z: {z}");

    // let _tup: () = (); // 이런 형식은 unit 이라고 부름

    // let _array = [1, 2, 3, 4, 5];
    // let _array: [i32; 3] = [1, 2, 3];
    // let _array = [1; 5]; // [1, 1, 1, 1, 1] 로 초기화 됨
    // let _array = [1, 2, 3, 4, 5];
    
    // let _first = _array[0];
    // let _second = _array[1];

    // println!("{_first} {_second}");

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
    let mut index = String::new();

    // 5 이상의 숫자를 입력하면 runtime error 가 발생함
    // C/CXX 와 같은 low-level 은 메모리 접근을 허용하지만, rust 는 index out of range exception 을 발생시키고 실행을 중지함.
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}

fn main() {
    variables();
    datatypes();
}
