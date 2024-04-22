// https://doc.rust-lang.org/book/ch05-03-method-syntax.html#methods-with-more-parameters

struct Rectangle {
    width: u32,
    height: u32
}

// 이 안에 들어있는 함수들을 Associated Function 이라고 함
// Associated function 이지만 메소드가 아닌 것들 (self 인자로 안 받는 것들)은 주로 생성자로도 쓰임, 이건건 주로 new 라는 이름을 가진 함수임 (이거 예약어 아님!)
impl Rectangle {
    // String::from 과 같이
    // self 가 첫 인자로 안 들어가는 함수는
    // Self::function_name 으로 부름
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

// 여러 impl 이 허용됨
impl Rectangle {
    // 파라미터가 여러개 들어가도 다른 언어랑 다른거 없음
    // 오히려 TypeScript 랑 비슷하게 생김
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let rect1 = Rectangle::square(40);

    let rect2 = Rectangle {
        width: 50,
        height: 30
    };

    let rect3 = Rectangle {
        width: 30,
        height: 20
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

/*
Struct는 커스텀 타입을 만들 수 있게 해줌
연관된 데이터들을 서로 연결되있게 해주고, 코드를 깔끔하게 함
impl 블럭에서 함수를 정의할 수도 있음.

하지만 Struct 만 커스텀 타입을 만들 수 있는게 ㅏㅇ님
enum 기능도 존재함
*/
