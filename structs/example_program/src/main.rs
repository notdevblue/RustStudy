// https://doc.rust-lang.org/book/ch05-02-example-structs.html

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area1(width1, height1)
    );

    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels",
        area2(rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area3(&rect2)
    );

    // println!("rect2 is {}", rect2); // Rectangle doesn't implement `std::fmt::Display`
    /*
    println! 메크로에서 {} 는 기본적으로 println! 이 Display라는 포맷을 사용하라고 함
    원어적 타입은 기본적으로 Display 를 구현함
    왜냐하면 `1`을 표시하는 방법은 그냥 `1`을 출력하는 되기 때문임

    하지만 struct 는 출력하는 방법의 가지수가 다양하기 때문에 어떻게 출력해야 하는지에 대해 약간의 문제가 있음
    이러한 문제 때문에, Rust는 우리가 뭘 원하는지 때려맞추지 않고, struct는 println! 메크로에서 {} 플레이스홀더를 이용한 Display의 구현을 가지고 있지 않음
    */

    /*
    에러를 읽다 보면 `{:?}` 를 사용해보라고 함
    :? 를 안에 넣게 되면, println! 에게 Debug 출력 포멧을 사용하라고 하는 것임
    Debug 트레잇은 개발자들에게 유용하게 struct 를 출력함 (디버깅하기 좋게 출력해줌)
    */
    // println!("rect2 is {:?}", rect2); // `Rectangle` doesn't implement `Debug`
    /*
    컴파일러가 #[derive(Debug)] 을 넣어보라고 함
    Rust는 디버깅 정보를 출력하기 위한 함수가 있음
    하지만 명시적으로 out in 해야 함
    */

    let rect3 = Rectangle2 {
        width: 30,
        height: 50
    };

    println!("rect3 is {:?}", rect3);
    /*
    struct가 크다면 :#? 를 사용할 수 있음
    */
    
    println!("rect3 is {:#?}", rect3);

    /*
    Debug 포멧을 이용해 출력하는 다른 방법은
    dbg! 메크로를 사용하는 것임
    dbg! 메크로가 호출된 파일 이름과 줄을 출력하고, 표현식의 결과값을 출력함.
    소유권을 가져가게 됨
    그리고 가져간 소유권을 반환값으로 돌려줌

    # stderr 에 출력함 #
    */

    let scale = 2;
    let rect4 = Rectangle2 {
        width: dbg!(30 * scale), // expression 의 결과를 반환 (소유권 반환을 응용)
        height: 50
    };
    
    dbg!(&rect4); // 레퍼런스 전달해서 소유권 못 가져가게 함
    // pretty한 디버그 출력 함
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}
/*
area 함수는 한 사각형의 넓이를 계산함
하지만 두개의 파라미터가 들어가게 되어 있음
묶어서 전달하면 더욱 명확할 것.
*/

fn area2(demensions: (u32, u32)) -> u32 {
    demensions.0 * demensions.1
}
/*
튜플로 데이터를 묶에서 보내게 되면 더 깔끔해짐
하지만 요소들은 이름을 가지지 않기 때문에 인덱스를 이용해 접금해야 함

너비와 높이를 섞는건 넓이 계산에서 문제가 되지 않음.
하지만 사각형을 그리게 된다면 문제가 됨.

0이 너비고, 1이 높이라는것을 항상 기억해야 함.
에러 내기 쉬워짐
*/

struct Rectangle {
    width: u32,
    height: u32
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
/*
이 함수는 rectangle 의 width, height 를 이용해서 넓이를 계산함
1번 함수에서 있던 문제점인 width와 height를 따로 주는 부분에서 두 값이 서로 연관되어있다고 표현이 가능해졌고,
인덱스를 통해 접근하던 2번 함수보다 width, height같은 자세한 이름을 이용해 접근하는 부분에서 명확성을 얻음
*/

#[derive(Debug)]
struct Rectangle2 {
    width: u32,
    height: u32
}

/*
area 함수는 아주 특정함

사각형(Rectangle 구조체)의 넓이만 계산함
이 기능이 Rectangle 구초제랑 더욱 가깝게 있었드면 좋을거 같음
왜냐하면 이 구조체 말고는 작동하지 않기 때문임.

area 함수를 area method 로 바꾸는 작업을 다음 챕터에서 함
*/