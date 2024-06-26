// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

/*
Rust에서 메모리 메니지먼트는 컴파일러 규칙을 통해 확인하는 오너쉽 시스템을 사용함.
규칙에 만족하지 않으면, 프로그램은 컴파일되지 않음
오너쉽 기능은 퍼포먼스 저하 주지 않음.

값이 stack 에 있냐, heap 에 있냐에 따라 언어의 행동이 달라짐

stack과 heap 코드가 실행하면서 사용이 가능한 메모리의 부분들임
하지만 서로간의 구조는 다름

## stack
stack은 (stack 자료 구조) stack 처럼 메모리가 저장되고 삭제됨
last in first out (first in last out) 구조

stack에 데이터를 넣는 것은 pushing onto the stack (스택에 밀어 넣는다)
stack에 데이터를 빼는 것은 popping off thet stack (스택에서 튀어 나오다)

stack에 들어가는 데이터들의 사이즈들은 알수 있고, 고정되있어야 함.
컴파일 시간에 데이터의 크기를 알수 없거나, 데이터의 크기가 변경이 될 가능성이 있다면 heap에 들어가야 함

## heap

heap은 덜 정리되어 있음.
heap에 데이터를 넣게 되면, 어떤 특정한 크기의 공간을 요청하게 됨.
메모리 할당자 (memory allocator) 는 특정한 크기 만큼의 큰 공간을 찾은 뒤, 사용중으로 표시해두고, 그 공간을 가리키는 포인터를 반환함. (int* myArray = new int[100]; 같은 느낌)
이 프로세스는 allocating on the heap (heap 에 할당한다)이라고 함. 그리고 가끔 allocating(할당한다) 이라고도 불림 (stack에 데이터 푸쉬하는건 allocating 이라고 안함)

heap을 가리키는 포인터의 크기는 알수 있고, 고정되있기 때문에 stack에 저장할 수 있음.
하지만 실제 데이터를 원하는 경우, 반드시 포인터를 따라가야 함.

레스토랑에 들어간다고 생각하면 편함.
레스토랑에 들어간 뒤, 몇명인지 알려주면, 직원이 방문한 인원이 모두 앉을 수 있는 빈 테이블을 찾고 안내해 줄 거임.
그리고, 누군가가 늦게 도착했다면, 직원을 통해 아까 온 사람들이 어디에 앉아있는지 물어볼 수 있음.

## speed (performance)

heap에 있는 데이터에 접근하는 것은 stack에 있는 데이터에 접근하는 것보다 느림.
heap에 있는 데이터를 접근하기 위해서는 pointer를 통해 접근해야 하기 때문임.

요츰 프로세서 (cpu)는 메모리에서 적게 이동할수록 빠름.
위에 예시에서 이어가자면, 직원이 레스토랑에서 주문을 받는다고 생각하는거임.
제일 효율적인 것은, 한 테이블에서 한번에 주문을 다 받고, 다음 테이블에 가서 주문을 받는 거임.
A 테이블에서 주문을 받고, B 테이블에서 주문을 받고, 다시 A 테이블에 가서 주문을 받고, 또 B 테이블에 가는거는 아주 느릴거임.

같은 맥락으로, 프로세서는 데이터가 서로서로 가까이 있으면 더욱 빠르게 작동하고, (stack에 있는 그대로) 멀리 떨어저 있을 수록 느리게 작동함 (heap 에 있을수 있음)

## stack 과 function

코드에서 함수를 호출하는 경우, 함수에 전달된 값들과 함수 내부의 변수들은 stack 에 할당됨.
(heap에 있는 데이터를 가리킬 가능성이 있는 포인터도 포함됨)
함수의 실행이 끝나게 되면, 값들은 stack에서 튀어 나오게 됨.

## 마무리

코드의 어느 부분에서 heap에 있는 어떠한 데이터를 사용하는지 기억하는 것,
최대한 heap에서 중복된 데이터를 줄이는 것,
그리고 heap에서 사용되지 않는 데이터를 삭제하여 메모리에 빈 공간을 확보하는 것들은 ownership이 담당하는 부분임.

ownership을 이해하고 나면, stack과 heap에 대해 그리 자주 생각하지 않아도 됨.
하지만 ownership의 주 목적이 heap의 데이터를 관리한다는 것을 알게 되면, 왜 이런 방식으로 작동하는지 알기 쉬워질 것.
*/


// ownership rules
/*
* 러스트에서 각각의 값은 owner를 가지고 있음
* 한번의 하나의 owner만 존재할 수 있음
* owner가 스코프 밖으로 나가게 되면, 값은 삭제될 것임
*/

fn main() {
    {
        // let mut _s = "Gentoo";
        // 대부분의 언어와 같은 스코프 작동 원리
        // 이 변수는 immutable
        // +, += 같은게 안 써짐
    }

    // let mut s = String::from("Install Gentoo");
    // heap에 저장됨

    // s.push_str(" and Remove Windows."); // string 에 리터럴 추가

    // println!("{}", s);

    // 왜 String은 가능한데 리터럴은 불가능한 것인가?
    // 그것은 두가지의 타입이 메모리를 서로 다르게 사용하기 때문임.

    /*
    스트링 리터럴은 컴파일 타임에 명확하게 알 수 있음.
    그렇기 때문에 문자열이 실행 가능한 파일에 하드코딩되어 들어감
    이러한 이유로 스트링 리터럴이 빠르고 효율적임
    하지만 이러한 속성은 스트링 리터럴의 불변성 덕분임.
    불행이도 바이너리에 컴파일 시간에 크기를 알 수 없고, 프로그램 실행 도중에 크기가 바뀔 가능성이 있는 메모리를 넣을 수는 없음.

    가변성과 크기 변화를 지원하기 위한 String 타입을 통해, 우리는 데이터를 저장하기 위해 컴파일 타임에 크기를 알 수 없는 메모리만큼을 heap에 할당해야 함.
    이는
    * 메모리는 런타임에서 메모리 할당자를 통해 요청되야 함.
    * `String`의 사용이 끝났을 시, 메모리 할당자에게 메모리를 반환할 방법이 필요함.
    을 의미함.

    첫번째 부분은 String::from 을 통해 해결되었음.
    이것의 구현은 필요한 메모리를 요청하는 것임.
    이건 대부분의 프로그래밍 언어에서 보편적임.

    하지만 두번째 부분은 조금 다름.
    가비지 컬랙터 (GC)가 있는 언어는 GC가 사용되지 않는 메모리를 추척하고 정리함. 그리고 우리는 이거에 대해 생각하지 않아도 됨.
    GC가 없는 대부분의 언어들은, 어떠한 메모리가 더이상 사용되지 않는지, 그리고 사용되지 않는 메모리를 명시적으로 할당 해제하는것이 프로그래머의 책임임; 위에서 String::from 으로 요청한 것 처럼.

    올바르게 프로그래머가 메모리 할당 해제 하는 작업은 역사적으로 계속 내려오는 어려운 프로그래밍적 문제임.
    만약 까먹으면, 메모리를 낭비하게 됨.
    너무 빨리 하면, 올바르지 못한 값을 가지게 됨.
    두번 하게 되면, 그것 또한 버그임.
    우리는 반드시 하나의 할당에 하나의 할당 해제가 필요함.

    Rust는 다른 방식을 택함.
    메모리를 잡고 있는 변수가 스코프를 벗어날 경우, 자동으로 메모리를 반환함
    */

    {
        // let _s = String::from("Install Gentoo"); // _s는 이 스코프에서 유효함

        // _s로 무언가 함
        // 으쌰으쌰
        // 영차영차
    }
    // 스코프 밖이니까 여기서부터는 _s가 유효하지 않음.

    /*
    String이 필요로 하는 메모리를 allocator에 반환하는 자연스러운 곳이 있음: _s가 스코프 밖으로 벗아나는 곳임.
    변수가 스코프 밖으로 벗어나게 되면, Rust는 특별한 함수를 호출함. drop 이라고 불림.
    String 클래스를 만든 사람이 drop 에다가 메모리를 반환하는 코드를 작성할 수 있음.
    그리고 위의 예시에선 Rust는 자동으로 닫는 중괄호 부분에서 drop 을 호출함.

    C++의 RAII 패턴이랑 상당히 동일함.
    
    이 패턴은 Rust 코드가 작성되는 방식의 지대한 영향을 미침.
    지금은 상당히 간단하게 보이지만, Heap에 할당한 데이터를 여러 변수가 사용하고 싶을 때 코드는 예상하지 못한 방식으로 실행될 수 있음.
    */

    // let _x = 5;
    // let _y = _x;
    // 값복사

    // let s1 = String::from("Gentoo");
    // let s2 = s1;
    // string
    /*
          s1
    -------------
    | ptr | 0x1 |
    | len | 5   |
    | cap | 5   |
    -------------
    이런 형식으로 되어 있음
    ptr => 0x1 = ['G', 'e', 'n', 't', 'o', 'o'] (heap 에 있음)

    복사하면 heap 영역에 있는 메모리도 복사되는게 아니라
    포인터가 복사됨
    s1 => 0x1, 5, 5
    s2 => 0x1, 5, 5
    같은 방식

    s1, s2가 스코프 나가면 메모리 헤제 함
    그런데 둘다 같은 메모리 가짐 => double free();

    그래서
    let s2 = s1; 이후에는
    Rust는 s1이 유효하지 않다고 판단하고 스코프 밖으로 나가도 아무런 처리를 하지 않음
    */

    // println!("Install {}", s1);
    // 컴파일 안 됨

    /*
    Rust는 std::move 같은 형식으로 데이터가 이동함

    그리고 rust는 암시적으로 깊은복사 절대 안함
    */

    // let s1 = String::from("Gentoo");
    // let s2 = s1.clone(); // 명시적 깊은 복사
    // println!("s1 = {}, s2 = {}", s1, s2);

    // let x = 5;
    // let y = x;
    // println!("x = {x}, y = {y}");
    /* 스택에 있는 것들은 복사하기 꽤 빠름
    Rust는 Copy 라는 어노테이션 있음
    스택에 저장되는 타입에게 붙일 수 있음
    타입이 Copy 구현하면 변수는 move 안 하고 복사됨

    타입이나 타입의 어느 한 부분이 Drop 특성을 구현했다면 Copy 구현은 불가능함
    타입이 스코프 밖으로 나갔을때 뭔가 특별한거 해야지만 Copy를 추가했다면 컴파일에러 남

    Copy는
    * 간단한 스칼라 값
    * 할당이나 Copy가 가능한 어떤 형태의 리소스
    이면 구현 가능함

    정수, boolean, 부동소수점, char, 튜플 (Copy 가능한 타입만으로 이루어진 => (i32, i32), (i32, String) 은 안됨)
    */

    // let s = String::from("Gentoo");
    // takes_ownership(s); // 변수에 할당하는 것과 비슷함. (매개변수니까...)
    // // println!("{}", s); // 컴파일 에러
    // // 예전에 String match 할때 안 되던 이슈가 오너쉽 문제로 인한 이슈인듯

    // let x = 5;
    // makes_copy(x); // 변수에 할당하는 것과 비슷함. (매개변수니까...)

    // let s1 = gives_ownership(); // 반환값으로 넘겨진 ownership을 s1에 넣음
    // let s2 = String::from("Install Gentoo");
    // let s3 = takes_and_gives_ownership_back(s2); // s2가 함수로 전달되며 오너쉽 잃음, 반환값으로 전달도니 오너쉽을 s3가 획득함

    // s3, s1 은 처리되지만, s2는 이동되었기 때문에 아무 처리 안 함

    // let s1 = String::from("Gentoo");

    // let (s2, len) = calculate_length(s1); // s1은 이동됨

    // println!("The length of '{}' is {}.", s2, len);


    /*
    매번 값 넘기고 반환값 쓰고 하는 것은 너무 번거롭기 때문에
    Rust는 오너쉽을 이동시키지 않고 값을 사용하게 해주는 기능인 "Reference" 가 있음
    */

}

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();

//     (s, length)
// }

// fn takes_ownership(s: String) {
//     println!("{}", s);
// } // s가 스코프를 벗어낫기 때문에 drop 호출됨

// fn makes_copy(i: i32) {
//     println!("{}", i);
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("Install Gentoo");

//     some_string
// }

// fn takes_and_gives_ownership_back(a_string: String) -> String {
//     a_string
// }