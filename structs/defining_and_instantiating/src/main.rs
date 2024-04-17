// https://doc.rust-lang.org/book/ch05-01-defining-structs.html

fn main() {
    println!("# struct defining and instantiating #\n");
    defining_and_instantiating();

    println!("\n# tuple structs #\n");
    tuple_structs();
}

fn tuple_structs() {
    // https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types
}

fn defining_and_instantiating() {
    let user1 = User {
        active: true,
        username: String::from("InstallGentoo"),
        email: String::from("HelloWorld@gentoo.org"),
        sign_in_count: 1
    };

    println!("{}, {}, {}, {}", user1.active, user1.username, user1.email, user1.sign_in_count);

    let mut user2 = User {
        active: true,
        username: String::from("Windows"),
        email: String::from("HelloWordl@gentoo.org"),
        sign_in_count: 1
    };
    // 인스턴스 전부 mutable 이어야 함
    // 특정한 필드만 mutable 로 하는 건 불가능함

    user2.username = String::from("Larry");

    println!("{}, {}, {}, {}", user2.active, user2.username, user2.email, user2.sign_in_count);

    let user = build_user(String::from("email@email.com"), String::from("HelloWorld"));
    println!("{}", user.email);

    let _user2 = User {
        active: user1.active,
        username: user1.username.clone(),
        email: String::from("another@email.com"),
        sign_in_count: user1.sign_in_count
    };
    // 이렇게도 만들 수 있지만

    let _user2 = User {
        email: String::from("another@email.com"),
        ..user1
        // .. 은 남은 필드를 의미함
        // 남은 필드는 user1의 값을 사용하라는 의미
        // 업데이트 문법은 '=' 처럼 사용되기 때문에
        // 데이터를 move 시킴
        // 그래서 업데이트 이후 user1.username의 소유권은 user2로 넘어갔기 때문에
        // 사용할 수 없음
        // stack only data만 user1에서 가져왔다면 copy trait 이 구현되었기 때문에, 문제 없이 사용이 가능함
    };
    // 이렇게 업데이트 문법을 사용해서 만들 수 있음

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // 필드랑 변수 이름이 같으면 생략이 됨
        email,
        sign_in_count: 1
    }
} // 반환형으로 사용 가능

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
