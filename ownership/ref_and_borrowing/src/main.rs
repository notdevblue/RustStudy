fn main() {
    // let s1 = String::from("Gentoo");
    // let len = calculate_length(&s1); // Rust에선 & 가 레퍼런스를 의미함
    // /*
    // &s1 문법은 레퍼런스를 생성함
    // 히지만 소유권을 가지진 않음
    // */

    // println!("The length of '{}' is {}.", s1, len);

    let s = String::from("Install");
    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(" Gentoo");
}

// fn calculate_length(s: &String) -> usize { // s는 String의 레퍼런스임
//     s.len()
// } // s가 스코프 밖으로 나가지만 소유권이 없으므로 파괴되지 않음
// // 이 행동을 Reference borrowing 이라고 함.
// // 현실에서 누군가가 가지고 있는 물건을 잠시 빌리고 다시 돌려주는 거임; 지산이 영원히 소유하지는 않고