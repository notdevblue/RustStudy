// https://doc.rust-lang.org/book/ch03-03-how-functions-work.html

fn main() {
    println!("Hello, world!");

    another_function();
    another_function_with_parameter(154);
    another_function_with_multiple_parameter(1.54, 'm');

    statement();
}

// 위에 정의하든 아레에 정의하든
// 스코프 안에 있으면 됨
fn another_function() { // Rust 는 snake case 를 사용함 
    println!("Install Gentoo.");
}

fn another_function_with_parameter(x: i32) {
    println!("The value of x is: {x}");
}

fn another_function_with_multiple_parameter(value: f64, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Statements: 값 반환 안하는 어떠한 Action 명령문들
// Expressions: 결과값 반환하는 것

fn statement() {
    let _y = 6; // 값을 반환하지 않음. Statement 임

    // let x = (let y = 10); // 변수 선언은 statement 이므로 다른 변수에 대입하는 statement 는 불가능함 (컴파일 에러 남)
    // expected expression, found `let` statement

    // let y = 10 은 값 반환이 없어서 x 에 대입할 뭔가가 없음
    // c 같은건 x = y = 10; 같은거 됨
    // 근데 Rust 는 안됨

    // 5 + 6 은 11이라는 값을 반환하기 때문에 Expression 임
    // Expression 은 statement 의 일부분일 수 있음
    let _y = 6;
    // 6은 값 6을 반환하는 expression 임

    let _y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {_y}");

    // 스코프 블럭 관련 하는 중
}
