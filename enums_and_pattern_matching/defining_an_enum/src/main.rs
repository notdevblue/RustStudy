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

fn route(ip_kind: &IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("Routed IPv4"),
        IpAddrKind::V6 => println!("Routed IPv6")
    }
}

fn print_ipaddr(ip_addr: &IpAddr) {
    route(&ip_addr.kind);
    println!("Address: {}", ip_addr.address);
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
}
