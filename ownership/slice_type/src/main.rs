// https://doc.rust-lang.org/book/ch04-03-slices.html
/*
slices 는 컬렉션의 연속되는 요소들을 컬랙션 통째를 가지는 것 대신 레퍼런스 할 수 있게 해줌
*/

fn main() {
    tedious_way();
    string_slices();
}

fn string_slices() {
    // // https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices

    // // string slice는 String 일부분의 레퍼런스임
    // let s = String::from("Install Gentoo");
    // let install = &s[0..7];
    // let gentoo = &s[8..14];
    // // [inclusive..exclusive]
    // // 복사해서 가지는것이 아닌
    // // 원레 오브젝트를 가리키는 포인터 형식임
    // // [8..14] 라면, s[8] 부터 6 (14 - 8)만큼의 길이를 가지는 슬라이스임
    // // { ptr-> s[8], len -> 6 }
    // println!("{}{}", install, gentoo);

    // let len = s.len();

    // // 0부터 시작하고 싶으면 앞에 시작 인덱스는 없어도 됨
    // let slice1 = &s[0..3];
    // let slice2 = &s[..3];
    // // 둘다 같은 의미
    // println!("{} {}", slice1, slice2);

    // // 반대로 끝까지 가고 싶으면 뒤에 인덱스도 없어도 됨
    // let slice1 = &s[8..len];
    // let slice2 = &s[8..];
    // // 둘다 같은 의미
    // println!("{} {}", slice1, slice2);

    // // 앞뒤 둘다 없엘수도 있음
    // let slice1 = &s[0..len];
    // let slice2 = &s[..];
    // // 둘다 같은 의미
    // println!("{} {}", slice1, slice2);

    // // string slice 인덱스는 UTF-8 문자 범위 내에서 사용되어야 함
    // // 멀타바이트 문자에서 사용한다면 터짐
    // // let multibyte = String::from("؃ ؃ ؃ ");
    // // let b = &multibyte[..1];
    // // println!("{b}");
    // // 이 섹션에선 ascii 문자만 사용한다고 가정

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
