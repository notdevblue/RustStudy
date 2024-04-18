// https://doc.rust-lang.org/book/ch05-01-defining-structs.html

fn main() {
    println!("# struct defining and instantiating #\n");
    defining_and_instantiating();

    println!("\n# other structs #\n");
    other_structs();

    println!("\n# ownership if struct data structs #\n");
    ownership_of_struct_data();
}

fn ownership_of_struct_data() {
    // https://doc.rust-lang.org/book/ch05-01-defining-structs.html#ownership-of-struct-data

    // 아레에서 &str 슬라이스 타입이 아닌 String 형식의 스트링 전부를 들고오는 방식을 탁함
    // 레퍼런스를 가지게 할 수도 있음
    // 하지만 이걸 위해서는 lifetime 라는 것을 사용해야 함
    // lifetime 은 struct 에서 레퍼런스로 가진 데이터가 struct 가 유효한 동안 유효한지 보장함

    // let user = User2 {
    //     email: "install@gentoo.always"
    // };
}

// struct User2 {
//     email: &str // 컴파일러가 lifetime 지정자가 필요하다고 불평할거임
// }
// 챕터10에서 고칠 것


fn other_structs() {
    // https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types

    let mycolor = Color(225, 123, 2);
    let mypoint = Point(1, 5, 4);
    // mycolor 과 mypoint 의 값은 다름.
    // 다른 tuple struct의 instance 이기 때문
    // 필드의 값이 같아도 사로 다른 인스턴스임
    // 이거를 제외하고는 튜플과 동일함.

    println!("{} {} {}", mycolor.0, mycolor.1, mycolor.2);
    
    println!("{} {} {}", mypoint.0, mypoint.1, mypoint.2);

    let sub = AlwaysEqual;
    let sub2 = AlwaysEqual;
    assert_eq!(sub, sub2);
    // 이 유닛 구조체의 인스턴스는 항상 동일함
}

// Unit-like struct
// trait 을 구현하고 싶은데 아무 데이터도 넣고 싶지 않을 때
// () 와 비슷하게 동작함
#[derive(PartialEq, Debug)]
struct AlwaysEqual;

// tuple struct
// tuple 에 이름을 넣어주고 싶을 때
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

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
