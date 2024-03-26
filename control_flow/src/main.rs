// https://doc.rust-lang.org/book/ch03-05-control-flow.html

fn main() {
    if_expressions();
    loops();
}

fn loops() {
    // let mut number = 0;

    // loop { // while 이랑 같음 (조건 못 넣는)
    //     number += 1;

    //     if number > 20 {
    //         println!("bye");
    //         break; // break 가능
    //     }

    //     if number % 2 != 0 {
    //         continue; // continue 가능
    //     }

    //     println!("number {number} is divisible by 2");
    // }

    // let mut counter = 0;

    // let result = loop { // expression 처럼 사용이 가능함
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2; // 여기에 return 처럼 값 전달하면 result 로 리턴
    //     }
    // };

    // println!("The result is {result}");

    // let mut count = 0;

    // let result = 'counting_up: loop { // loop 에 label 을 넣을 수 있음, 그리고 위에 expression 처럼 사용 가능한거도 가능함
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break; // 이 loop 를 break 함
    //         }
    //         if count == 2 {
    //             break 'counting_up "bonjour"; // 시발 이게 왜 가능해
    //             // 'counting_up label 을 가진 loop 을 break 함
    //             // 언어 이상해
    //         }

    //         remaining -= 1;
    //     };

    //     count += 1;
    // };

    // println!("End count = {count}, result: {result}");

    // let mut number = 3;

    // while number != 0 { // 진짜 while 문
    //     println!("{number}!");

    //     number -= 1;
    // }

    // println!("LIFTOFF!!!");

    // let a = [10, 20, 30, 40, 50];

    // let mut index = 0;
    // while index < 5 {
    //     println!("the value is: {}", a[index]);

    //     index += 1;
    // }
    // // while 문을 이용한 array 출력

    // let a = [10, 20, 30, 40, 50];

    // for element in a { // foreach 느낌
    //     println!("the value is: {element}");
    // }

    // // for 문을 사용하면, a 배열의 요소 수를 바꿔도 문제가 되지 않음
    // // n 번 반복하고 싶을때도, 대부분의 러스트 유저는 for 문을 사용함
    // // standard libaray 의 Range 를 사용하면 됨.
    // // start inclusive, end exclusive.

    // for number in (1..4).rev() {
    //     println!("{number}!");
    // }
    // println!("LIFTOFF!!!");
    // 위에서 작성한 카운트다운 코드를 for 문으로 바꾼 것

    

}

fn if_expressions() {
    // let number = 6;

    // if number < 5 {
    //     println!("condition was true");
    //     // 요 괄호 안에 있는 코드들을 arms 이라고 부르기도 함
    // } else {
    //     println!("condition was false");
    // }
    
    // if number { // 변수만 사용되면 boolean 타입을 기대함
    //     println!("number was seven");
    // }

    // if number != 0 { // 0이 아닐때 실행시키고 싶으면 이렇게 해야 함
    //     println!("number was something other than zero");
    // }

    // if number % 4 == 0 {
    //     println!("nubmer is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // } // 이런 형식으로 코드가 진행된다면 match 를 사용하도록 코드를 리팩토링 하는 것을 추천함

    // if 는 expression 이기 때문에(?) rvalue 로 올수 있음. (이게 왜 expression 으로 처리가 됨?)

    // let condition = true;
    // let number = if condition { println!("hello world"); 5 } else { 6 }; // ???????
    // // 삼항연산자 비슷한 무언가 느낌
    // // 좀 강한 삼항연산자

    // println!("The value of number is: {number}");

    // let condition = true;

    // let number = if condition { 5 } else { "six" };
    // // if returns i32, but else returns &str
    // // type mismatch

    // println!("The value of number is: {number}");

}
