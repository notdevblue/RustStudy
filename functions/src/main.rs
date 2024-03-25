// https://doc.rust-lang.org/book/ch03-03-how-functions-work.html

fn main() {
    // println!("Hello, world!");

    // another_function();
    // another_function_with_parameter(154);
    // another_function_with_multiple_parameter(1.54, 'm');

    // statement();

    // let x = five();
    // println!("The value of x is: {x}");

    let x = plus_one(153);
    println!("The value of x is: {x}");
}

// fn five() -> i32 {
//     5 // 이거 반환값임
// }

fn plus_one(x: i32) -> i32 {
    x + 1

    // return x + 1; // 명시적으로 return 을 할 수도 있음

    // x + 1;
    // 세미콜론을 붙이게 되면, Expression 에서 statement 가 되고
    // i32 타입이 아닌 유닛 '()' 을 리턴하게 되므로 컴파일 에러가 뜸
}


// 위에 정의하든 아레에 정의하든
// 스코프 안에 있으면 됨
// fn another_function() { // Rust 는 snake case 를 사용함 
//     println!("Install Gentoo.");
// }

// fn another_function_with_parameter(x: i32) {
//     println!("The value of x is: {x}");
// }

// fn another_function_with_multiple_parameter(value: f64, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// Statements: 값 반환 안하는 어떠한 Action 명령문들
// Expressions: 결과값 반환하는 것

// fn statement() {
//     let _y = 6; // 값을 반환하지 않음. Statement 임

//     // let x = (let y = 10); // 변수 선언은 statement 이므로 다른 변수에 대입하는 statement 는 불가능함 (컴파일 에러 남)
//     // expected expression, found `let` statement

//     // let y = 10 은 값 반환이 없어서 x 에 대입할 뭔가가 없음
//     // c 같은건 x = y = 10; 같은거 됨
//     // 근데 Rust 는 안됨

//     // 5 + 6 은 11이라는 값을 반환하기 때문에 Expression 임
//     // Expression 은 statement 의 일부분일 수 있음
//     // let _y = 6;
//     // 6은 값 6을 반환하는 expression 임

//     let _y = {
//         let x = 3;
//         x + 1 // 이거 반환함
//         // Expression 은 세미콜론 없음
//         // 세미콜론 넣으면 Statement 되어 값 반환 안함
//     };

//     println!("The value of y is: {_y}");

// }

