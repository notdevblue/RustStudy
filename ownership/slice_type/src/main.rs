// https://doc.rust-lang.org/book/ch04-03-slices.html
/*
slices 는 컬렉션의 연속되는 요소들을 컬랙션 통째를 가지는 것 대신 레퍼런스 할 수 있게 해줌


*/


fn main() {
    let s = String::from("Install Gentoo");

    let len = first_word(&s);

    println!("{}", len);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        println!("{}", item as char);
        
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
