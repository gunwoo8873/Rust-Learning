fn main() {
    owner();
    ex_copy_variable();
    ex_move_var();
    borrowing();
    slice_fn();
}

// Note : ownership 은 Rust 의 메모리 관리법을 지배하는 규칙 모음이다. 어떤 언어들은 GC[Garbage Collector]으로 더 이상
//        사용하지 않는 메모리를 정기적으로 찾는 방식을 채택하고 혹은 직접 사용자가 메모리를 할당 및 해제 하는 방식을 채택했다.
//        Rust 는 이 방식을 개선하여 GC가 없이 메모리 안정성을 보장하도록 시스템을 설계 했다.

fn owner() { // { } = Scope 라고 칭하며 코드가 수행을 하는 동안 유효한 블럭이다.
    let mut lifecode = String::from("Code");
    lifecode.push_str("Test");
    println!("{}", lifecode);
} // 수행이 끝난 코드는 할당된 메모리가 해제가 되기 때문에 더 이상 유효한 코드가 아니다.

fn ex_copy_variable() {
    let x = String::from("Copy Variable"); // 초기 변수 [14 byte String]
    let y = x; // Pointer 이동 = x [유효하지 않는 변수] => y [유효한 변수]
    let z = y.clone(); // 허용 데이터 타입 : i32, bool, char, f32/64 [단 동일한 타입만 Copy 가 가능]
    println!("z = {}", z);
}
// 이 방식의 메모리 이동은 연산이 느리기 때문에 권장하지 않는다.

fn ex_move_var() {
    let x = give_fn();
    let y = x;
    let (z, len) = take_give_fn(y);
    println!("z = {}, len = {}", z, len);
} // x 의 값이 take_function 으로 이동되었기 때문에 컴파일이 끝나기 전까진 유요한 코드로 남아있는다.

// fn take_function(x: String) {
//     println!("x = {}", x);
// }

fn give_fn() -> String {
    let take_function = String::from("Return Variable");
    take_function
}

fn take_give_fn(_return: String) -> (String, usize) {
    let length = _return.len();
    (_return, length)
}

fn borrowing() {
    let mut x = String::from("Borrowing");
    borrowing_take_fn(&mut x);
    println!("{}",x);
}

fn borrowing_take_fn(x: &mut String) {
    x.push_str(", Return Variable");
}
// Note : & 을 사용하여 해당 값을 소유하지 않지만 참조자를 생성하여 LifeTime 이 끝나더 라도 유효한 코드로 남아 있게 된다.

fn slice_fn() {
    let mut s = String::from("Return Slice");
    let len = slice(&s);
    s.clear();
}

fn slice(s: &String) -> usize {
    let byte = s.as_bytes(); // Byte 배열로 변환 처리

    for (i, &item) in byte.iter().enumerate() { // .iter() = iterator[반복자] 메서드 추가
        if item == b' ' {
            return i;
        }
    }

    s.len()
}