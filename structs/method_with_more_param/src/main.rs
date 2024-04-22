// https://doc.rust-lang.org/book/ch05-03-method-syntax.html#methods-with-more-parameters

struct Rectangle {
    width: u32,
    heigth: u32
}

impl Rectangle {
    // 다른 언어랑 다른거 없음
    // 오히려 TypeScript 랑 비슷하게 생김
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.heigth >= other.heigth
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        heigth: 54
    };

    let rect2 = Rectangle {
        width: 50,
        heigth: 30
    };

    let rect3 = Rectangle {
        width: 30,
        heigth: 20
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
