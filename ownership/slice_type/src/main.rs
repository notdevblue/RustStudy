// https://doc.rust-lang.org/book/ch04-03-slices.html
/*
slices 는 컬렉션의 연속되는 요소들을 컬랙션 통째를 가지는 것 대신 레퍼런스 할 수 있게 해줌


*/


fn main() {
    tedious_way();
    string_slices();
}

fn string_slices() {

}

fn tedious_way() {
    // let mut s = String::from("Install Gentoo");
    
    // let word = first_word(&s);
    // /*
    // usize를 반환함
    // &String과 관련된 값임, String과 별개의 값이기 때문에 미래에 유효한지 보장되지 않음
    // */
    
    // s.clear();
    // // String 이 비워진 경우, word는 7을 가지고 있지만, 의미있게 사용이 불가능함.
    // // second_word 라는 함수를 만들게 되면, 반환 타입은 (usize, usize) 가 됨.
    // // 상당히 조잡하고 에러에 취약함
    
    // println!("{}", word);
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes(); // element by element

//     // iterator는 13장에서 다룸
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }
