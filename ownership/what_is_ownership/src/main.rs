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

stack에 데이터를 넣는 것은 pushing onto the stack
stack에 데이터를 빼는 것은 popping off thet stack

stack에 들어가는 데이터들의 사이즈들은 알수 있고, 고정되있어야 함.
컴파일 시간에 데이터의 크기를 알수 없거나, 데이터의 크기가 변경이 될 가능성이 있다면 heap에 들어가야 함
*/

fn main() {
    println!("Hello, world!");
}
