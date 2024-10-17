fn main() {
    owner();
    ex_move_variable();
}

/*
ownership 은 Rust 의 메모리 관리법을 지배하는 규칙 모음이다. 어떤 언어들은 GC[Garbage Collector]으로 더 이상
사용하지 않는 메모리를 정기적으로 찾는 방식을 채택하고 혹은 직접 사용자가 메모리를 할당 및 해제 하는 방식을 채택했다.
Rust 는 이 방식을 개선하여 GC가 없이 메모리 안정성을 보장하도록 시스템을 설계 했다.
*/

fn owner() { // { } = Scope 라고 칭하며 코드가 수행을 하는 동안 유효한 블럭이다.
    let mut lifecode = String::from("Code");
    lifecode.push_str("Test");
    println!("{}", lifecode);
} // 수행이 끝난 코드는 할당된 메모리가 해제가 되기 때문에 더 이상 유효한 코드가 아니다.

fn ex_move_variable() {
    let x = String::from("TestMemoryMove"); // 초기 변수 [14 byte String]
    let y = x; // Pointer 이동 = x [유효하지 않는 변수] => y [유효한 변수]
    let z = y.clone(); // 허용 데이터 타입 : i32, bool, char, f32/64 [단 동일한 타입만 Copy 가 가능]
    println!("z = {}", z);

}
// 이 방식의 메모리 이동은 연산이 느리기 때문에 권장하지 않는다.

fn borrowing() {

}

fn slice() {

}