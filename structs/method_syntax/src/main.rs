// https://doc.rust-lang.org/book/ch05-03-method-syntax.html

/*
method는 함수와 비슷함
fn 키워드를 이용해 정의하고, 파라미터와 반환값이 있을 수 있음

method는 함수와는 다르게 struct (enum 일수도 있고 trait object 일수도 있고)의 안에 정의됨
그리고 맨 처음 파라미터는 항상 self임
method가 호출한 struct의 인스턴스를 의미함
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

/*
Rectangle 안에 함수를 정의하기 위해서 Rectangle 의 impl 블럭에서 시작함
impl 블럭 안의 모든 것은 Rectangle 타입과 관련됨
area function을 블럭 안으로 옮기고, self 파라미터를 추가함
나머지는 다른 언어와 비슷함

area의 시그니처를 보면 rectangle: &Rectangle 이 아닌 &self 를 사용함
self 은 self: &Self 를 줄인거임
impl 블럭 안에서 Self 은 impl 블럭의 대상을 말함
method는 반드시 self 라는 이름을 가졌거나, Self 타입인 파라미터가 첫번째로 존재해야 함.

Rust는 self 라는 이름을 바꿀 수 있게 해주는 거임

위에서 우리는 & 를 붙임, 왜냐하면 인스턴스를 잠시 빌려오는 것이기 때문임
다른 파라미터와 동일하게 method는 self의 오너쉽을 가져갈 수 있고, mutably borrow, immutably borrow 도 가능함

&self를 택한 이유는 소유권을 가지고 싶지 않고, 값을 바꾸고 싶지도 않았기 때문임
만약 값을 바꾸고 싶었다면 &mut self를 첫번째 파라미터로 사용하면 됨

self만 사용하여서 소유권을 가져가는 파라미터는 흔하게 찾아보기 힘듬.
이 기술은 self를 다른 무언가로 바꾸고 난 뒤, 실수로 원본을 호출하는걸 막기 위해 주로 사용됨.

함수 대신 메소드를 사용하는 주요 이유와 메소드 문법을 제공하는 이유, 그리고 모든 메소드 시그니처에 self 를 반복해도 되지 않아도 되는 이유는
조직화를 위한 것임

인스턴스에서 가능한 모든 것들을 impl 블럭에 넣어서 미래의 유저들이 Rectangle 이 뭘 할수 있는지 모든 코드를 뒤지는거 보다 impl 블럭만 찾으면 좋기 때문
*/

// 메소드 이름이 구조체의 필드 이름과 같아도 됨
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}
/*
width 을 괄호 없이 사용하게 되면 Rust는 우리가 구조체의 width 필드를 접근한다는걸 알수 있고,
괄호를 붙이게 되면 구조체의 메소드인 width() 를 접근한다는걸 알수 있음

필드와 같은 이름을 가진 메소드가 필드의 값만 리턴하고 아무것도 하지 않으면 getter 라고 부름
*/

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}


// -> 는 어디로 사라졌는가
/*
Rust는 automatic referencing과 dereferencing이 존재함

Rust는 메소드의 self 시그니처랑 맞추기 위해 자동으로 &, &mut, * 등을 넣어줌

rect1.area() 를 호출하는 것은
(&rect1).area() 와 같음
*/