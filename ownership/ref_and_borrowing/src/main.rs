fn main() {
    mutable_reference_basic();
    restriction();
    dangling();
}

fn dangling() {
    // https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references
}

fn restriction() {
    // let mut s = String::from("Gentoo");

    // let r1 = &mut s;
    // let r2 = &mut s;
    // 컴파일 단계에서 race condition 막음

    // println!("{}, {}", r1, r2);

    // let mut s = String::from("Gentoo");
    // {
    //     let _r1 = &mut s;
    // }
    // let _r2 = &mut s; // _r1은 더이상 존재하지 않기 때문에 문제 없이 만들 수 있음

    // let mut s = String::from("Gentoo");

    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s; // 섞어서 쓰는 거도 안 됨. r1, r2 는 이게 바뀔 거라고 생각하고 있지 않아서 그럼

    // println!("{}, {}, and {}", r1, r2, r3);

    // let mut s = String::from("Hello Gentoo");

    // let r1 = &s;
    // let r2 = &s;

    // println!("{} and {}", r1, r2);
    // // 레퍼런스 스코프는 처음에 생성되었을 때 부터
    // // 마지막으로 사용된 지점까지임.
    // // 그래서 r1, r2 더 안 쓰이니까
    // // 아레쪽에 r3 에 mutable reference 할당이 가능한 것
    // // 스코프가 겹치지 않기 때문에.

    // let r3 = &mut s;
    // println!("{}", r3);
}


fn mutable_reference_basic() {
    // let s1 = String::from("Gentoo");
    // let len = calculate_length(&s1); // Rust에선 & 가 레퍼런스를 의미함
    // /*
    // &s1 문법은 레퍼런스를 생성함
    // 히지만 소유권을 가지진 않음
    // */
    
    // println!("The length of '{}' is {}.", s1, len);
    
    // let s = String::from("Install");
    // change(&s);
    
    // let mut s = String::from("Install");
    
    // change(&mut s); // mutable reference
    // println!("{}", s);
}

// fn change(some_string: &mut String) { // mutable reference
//     some_string.push_str(" Gentoo");
// }

// fn change(some_string: &String) {
//     some_string.push_str(" Gentoo"); // 기본적으로 레퍼런스도 immutable 임
// }

// fn calculate_length(s: &String) -> usize { // s는 String의 레퍼런스임
//     s.len()
// } // s가 스코프 밖으로 나가지만 소유권이 없으므로 파괴되지 않음
// // 이 행동을 Reference borrowing 이라고 함.
// // 현실에서 누군가가 가지고 있는 물건을 잠시 빌리고 다시 돌려주는 거임; 지산이 영원히 소유하지는 않고.