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
    
}
*/


fn main() {
    println!("Hello, world!");
}
