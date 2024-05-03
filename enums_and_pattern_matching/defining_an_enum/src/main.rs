// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html

/*
Enum 은 가능한 변형들을 열거하는 방식으로 타입을 정의할 수 있게 해줌

Struct는 관련된 필드랑 데이터를 한데 묶는 방식을 제공할 때, (Rectangle 의 width 와 height 같은)

Enum은 어느 값이 가능한 값들의 묶음중 하나라는것을 의미함
예를 들어 Rectangle 은 Circle과 Triangle도 포함되어 있는 모양 타입의 가능한 값중 하나라는걸 의미할 수 있음

이런 기능을 위해 Rust는 이러한 가능성들을 enum 으로 인코딩함

enum이 struct보다 유용할 수 있는 케이스 (예시)
IP 주소 관련된 뭔가 할 때, v4와 v6가 있음
이 두개만이 가능한 범위이기 때문에, 이것을 열거할수 있음

IP 주소는 v4일수 있고 v6일수 있지만 동시에 두개 다 일순 없음
enum값은 하나만 가질 수 있기 때문에 IP주소는 enum구조에 적합함

v4와 v6둘다 기본적으론 IP 주소기 때문에 같은 타입으로 다뤄쟈아 함

이 개념을 아레에서 코드로 표현함
*/

enum IpAddrKind {
    V4,
    V6,
}
// IpAddrKind enum은 이제 코드 어디에서든지 사용이 가능한 커스텀 타입임
/*
enum 안에 있는 요소들을 :: 를 통해 접근
namespaced 되어 들어가 있는거임
*/


// 아직 IP 주소 자체를 저장하는 방법은 없기 때문에, 지금까지 배운 방법으로는 구조체를 통해 저장하는 방법을 택할 수 있음.
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
// 이러한 개념을 enum만 통해 한다면 더 쉬움
// enum 요소에 값을 넣을 수 있음. (?왜됨)

enum IpAddrEnum {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_kind: &IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("Routed IPv4"),
        IpAddrKind::V6 => println!("Routed IPv6"),
    }
}

fn print_ipaddr(ip_addr: &IpAddr) {
    route(&ip_addr.kind);
    println!("Address: {}", ip_addr.address);
}

fn print_ipaddr_enum(ip_addr: &IpAddrEnum) {
    match ip_addr {
        IpAddrEnum::V4(one, two, three, four) => println!("Routed IpV4, address: {one}.{two}.{three}.{four}"),
        IpAddrEnum::V6(ip) => println!("Routed IpV4, address: {ip}"),
    }
} // 안에 있는 요소도 이렇게 접근이 됨

enum Message {
    Quit,
    // 아무 데이터랑도 연관되지 않음
    
    Move { x: i32, y: i32 },
    // Struct 처럼 이름을 가진 필드가 있음

    Write(String),
    // 하나의 String을 포함함

    ChangeColor(i32, i32, i32),
    // 세가지의 i32값을 포함함
}
/*
enum을 이러한 방식으로 정의하는 것은 여러 구조체를 정의하는거랑 비슷함
하지만 enum은 struct 키워드를 사용하지 않고, Message 타입 안에 enum 변형들이 묶여있다는 것임.
*/

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32, i32);
// 이렇게 구조체를 정의하면 각기 다 다른 타입이라, 이것들을 받는 함수를 정의하기 어려움.
// 하지만 Message enum 안에 변형들은 일단 Message 타입이기 때문에 쉬움

/*
enum과 구조체의 또다른 공통점은 메소드를 impl 블럭 안에서 구현할 수 있다는 것임
(이게 왜 enum인지 모르겠다 이젠)
*/

impl Message {
    fn call(&self) {
        println!("Method called");
    }
}

fn main() {
    // route(&IpAddrKind::V4);
    // route(&IpAddrKind::V6);
    // 이런 방식으로 넘길 수 있음

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    print_ipaddr(&home);
    print_ipaddr(&loopback);

    println!("");

    let home = IpAddrEnum::V4(127, 0, 0, 1);
    let loopback = IpAddrEnum::V6(String::from("::0"));
    // enum의 요소가 인스턴스 생성자가 됨
    // 그리고 서로 다른 타입을 가질 수 있음
    // 사실 IP 저장하는건 너무 흔해서 스텐다드 라이브러리에 이미 있음
    // IpAddr.
    // enum안에 데이터는 struct도 들어가고, 또 다른 enum이 들어갈 수도 있음

    print_ipaddr_enum(&home);
    print_ipaddr_enum(&loopback);

    let msg = Message::Move { x: 1, y: 2 };
    msg.call(); // msg가 메소드에 전달되는 self임
}
