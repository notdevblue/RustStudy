// https://doc.rust-lang.org/book/ch05-03-method-syntax.html#methods-with-more-parameters

struct Rectangle {
    width: u32,
    heigth: u32
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
        width: 40,
        heigth: 20
    };

    println!("Can rect1 hold rect2? {}", false);
    println!("Can rect1 hold rect3? {}", false);
}
