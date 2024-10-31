pub fn main() {
    integer_type();
    float_type();
}

/*
 * 모든 값은 특정한 데이터 타입을 가지고 있으며, 해당 데이터로 작업하는 방법을 알릴 수 있도록 어떤 종류의 데이터가
 * 지정되어 있는지 알려준다. 스칼라 타입과 복합 타입으로 두 가지의 부분 집합으로 나누어져 있다.
 */

fn integer_type() {
    let signed_number: i32 = 15; // -2^n-1 ~ 2^n-1
    println!("signed_number : {}", signed_number);

    let unsigned_number: u32 = 15; // -2^7-1 ~ 2^7-1
    println!("unsigned_number : {}", unsigned_number);
}
/*
 * 정수형 타입 [Integer Typed]
 * 8 ~ 128 bit
 * Signed : [i8, i16, i32, i64, i128] = -2^n-1 ~ 2^n-1
 * Unsigned : [u8, u16, u32, u64, u128] = -2^7-1 ~ 2^7-1
 */

fn float_type() {
    let float_number: f32 = 20.000; // 32 bit
    println!("float_number : {}", float_number);

    let float_number: f64 = 20.000; // 64 bit
    println!("float_number : {}", float_number);

    let float_number_default = 20.000; // 64 bit = default
    println!("float_number_default : {}", float_number_default);
}
/*
 * 부동소수점 타입 [Float Typed]
 * 부동소수점은 IEEE-754 표준을 따라 f32는 1배수 정밀도이지만, f64는 2배수 정밀도를 가지고 있다.
 */