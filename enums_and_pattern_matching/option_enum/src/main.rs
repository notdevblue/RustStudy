// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values

/*
Option 타입은 값이 있거나 없거나를 나타낼때 사용됨

버이있지 않은 리스트의 첫번째 요소를 가져오면 값을 받아옴
비어있는 리스트의 첫번째 요소를 가져오면 아무것도 없음

이 개념은 컴파일러가 코드에서 모든 경우의 수를 헨들링하는지 확인이 가능하기 때문에
다른 언어에선 매우 흔한 버그를 사전에 막을 수 있음

Rust는 null 이 없음

null값의 문제는 null값을 not-null값 인것처럼 사용하게 되면 에러가 남
왜냐하면 null이나 notnull 프로퍼티는 퍼저나가기 때문임
아주 쉽게 이러한 에러를 낼 수 있음

하지만 null의 개념 자체는 유용함
null은 값이 현재 유요하지 않거나 어떠한 이유로 없다는 의미

러스트의 구현 개념 때문에 이러한 문제는 딱히 중요하지 않음
Rust 는 null이 없지만, 위의 개념을 대신할 enum인 Option<T> 가 존재함

enum Option<T> {
    None,
    Some(T),
}
(재정의할수는 없으니까..)

Option<T>는 기본적으로 포함됨 (prelude 에 있음)
Some, None 을 Option:: 없이 사용도 가능함

<T> 는 제네릭 타입 (다른 언어랑 같음)
*/

fn print_enum<T>(e: Option<T>) where T : std::fmt::Display {
    match e {
        Some(v) => println!("{v}"),
        None => println!("Absent!")
    }
}

fn main() {
    let some_char = Some('5');
    let some_number = Some(5);
    let absent_number: Option<i32> = None; // Some 에서는 값 타입에 따른 추정이 가능했지만 여기는 아니니 명시적으로 타입 지정 해야 함
    print_enum(some_char);
    print_enum(some_number);
    print_enum(absent_number);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y;
    /*
    Option<T> 와 T 는 서로 다른 타입이기 때문에 위와 같은 행동이 안됨
    Option<T> 가 유효한 값 인것 처럼 사용하지 못하게 컴파일러가 막음
    
    이것이 Null대신 Option<T> 가 가지는 장점임
    */
}

/*
Rust는 i8에 Option<i8> 를 더하는 것을 이해할 수 없음, 다른 타입이기 때문

i8같은 타입이 있을 때, 컴파일러는 항상 유효한 값을 가지도록 보장함, null인지 확인하지 않아도 됨
Option<T> 같은 타입일 때만 값이 없는 가능성을 걱정해야 하고, 컴파일러가 값이 없을때의 처리를 했는지 확실히 확인함

다른말로는, T 타입 연산을 하기 위해선 Option<T>를 T로 변환해야 함

not-null값을 잘못 생각하는 위험을 제거하는 것은 코드를 더욱 확실하게 만듬
Option<T>를 이용해서 명시적으로 null을 처리하게 함

Rust 코드의 안전성을 증가시키기 위해 이러한 디자인 선택을 함.

https://doc.rust-lang.org/std/option/enum.Option.html 를 통해 Option<T>의 메소드들을 볼 수 있음

Option<T>를 사용하기 위해선, Option<T>의 variant들을 코드에서 처리해야 함
Some(T) 의 T 를 사용할때와, None 일때의 처리 등등..
match 표현식은 위 상황을 표현하기 아주 좋은 컨트롤 플로우를 만듬
enum variant 에 맞는 코드를 실행하고, 내부의 값을 사용할 수 있음
*/
