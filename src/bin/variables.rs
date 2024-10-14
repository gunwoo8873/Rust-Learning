pub fn main() {
    immutable_variable();
    mutable_variable();
    const_variable();
}

fn immutable_variable() {
    let immutable_number = 15;
    println!("{}", immutable_number);

    // a = 20; // Error
    // println!("{}", a);
}
/*
 * 변수는 기본적으로 { } 내에서만 유효하며 if문 { } 안에서 선언된 변수도 if문 안에서만 사용 가능하다.
 */

fn mutable_variable() {
    let mut mutable_number = 20;
    println!("{}", mutable_number);

    mutable_number = 50;
    println!("{}", mutable_number);
}
/*
 * let 은 기본적으로 값의 업데이트가 불가능한 시스템으로 형성 되어 있다.
 * 값을 업데이트 하기 위해선 mutable 키워드인 mut 를 사용하여 값의 업데이트가 진행이 가능
 */

const CONST_NUMBER_A: i32 = 100; // 보통 대문자를 이름을 작성하는 것이 관례
fn const_variable() {
    println!("{}", CONST_NUMBER_A);

    const CONST_NUMBER_B: f64 = 3.14;
    println!("{}", CONST_NUMBER_B);
}
/*
 * const 는 Immutable 같은 특징을 가지고 있지만 mut 와 같이 사용이 불가능 하다.
 * 반드시 값의 타입을 명시를 해야 하는 문제가 있지만 전역및 내부의 Scope 에서 선언이 가능하다.
 */