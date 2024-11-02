pub fn main() {
    integer_type();
    float_type();
}
// Note : 모든 값은 특정한 데이터 타입을 가지고 있으며, 해당 데이터로 작업하는 방법을 알릴 수 있도록 어떤 종류의 데이터가
//        지정되어 있는지 알려준다. 스칼라 타입과 복합 타입으로 두 가지의 부분 집합으로 나누어져 있다.

struct IntegerDatatype
{
    signed: i32, // -2^n-1 ~ 2^n-1
    unsigned: u32, // -2^7-1 ~ 2^7-1
}

fn integer_type()
{
    let int_datatype = IntegerDatatype
    {
        signed: -1,
        unsigned: 0,
    };

    println!("Signed : {}", int_datatype.signed);
    println!("Unsigned : {}", int_datatype.unsigned);
}
// Note : 정수형 타입 [Integer Typed]
//        8 ~ 128 bit
//        Signed : [i8, i16, i32, i64, i128] = -2^n-1 ~ 2^n-1
//        Unsigned : [u8, u16, u32, u64, u128] = -2^7-1 ~ 2^7-1

struct FloatDatatype
{
    f32_float: f32, // 32 bit
    f64_float: f64, // 64 bit [Default]
}

fn float_type()
{
    let flat_datatype = FloatDatatype
    {
        f32_float: 20.000,
        f64_float: 20.000,
    };

    println!("Float f32 Datatype : {:.2}", flat_datatype.f32_float);
    println!("Float f64 Datatype : {:.1}", flat_datatype.f64_float);
}

// Note : 부동소수점 타입 [Float Typed]
//        기본적으로 Float은 2배수 정밀도로 설정되어 있으니 별도로 f32를 명시필요 하다.
//        부동소수점은 IEEE-754 표준을 따라 f32는 1배수 정밀도이지만, f64는 2배수 정밀도를 가지고 있다.
// Link : https://ko.wikipedia.org/wiki/IEEE_754 [출처 = 위키백과 : IEEE-754]