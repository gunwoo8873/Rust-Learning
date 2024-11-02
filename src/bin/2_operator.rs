pub fn main() {
    elementary_arithmetic();
    boolean();
    string();
    tuple();
    array();
}

struct Int
{
    num_a: i32,
    num_b: i32,
}

impl Int
{
    fn sum(&self) -> i32
    {
        self.num_a + self.num_b
    }

    fn difference(&self) -> i32
    {
        self.num_a - self.num_b
    }

    fn product(&self) -> i32
    {
        self.num_a * self.num_b
    }

    fn quotient(&self) -> i32
    {
        self.num_a / self.num_b
    }

    fn remainder(&self) -> i32
    {
        self.num_a % self.num_b
    }
}

fn elementary_arithmetic() {
    let sum_result = Int {num_a: 10, num_b: 20};
    println!("Sum : {}", sum_result.sum());

    let diff_result = Int {num_a: 10, num_b: 20};
    println!("Diff : {}", diff_result.difference());

    let product_result = Int {num_a: 10, num_b: 20};
    println!("Product : {}", product_result.product());

    let quotient_result = Int {num_a: 10, num_b: 20};
    println!("Quotient : {}", quotient_result.quotient());

    let remainder_result = Int {num_a: 10, num_b: 20};
    println!("Remainder : {}", remainder_result.remainder());
}

fn boolean() {
    let t = true;
    println!("true : {}", t);

    let f = false;
    println!("false : {}", f);
}

fn string() {
    let c = 'c';
    println!("c : {}", c);

    let z = 'Z';
    println!("z : {}", z);

    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat : {}", heart_eyed_cat);
}
// Note : char 타입은 4byte 크기를 가지고 유니코드 스칼라값을 표현한다.
//        ASCII 보다 많은 값을 표현할 수 있어 다양한 언어, 이모지, 0인 공백 문자 모두가 유효한 타입이다.
//        유니코드 스칼라값의 범위는 U+0000 ~ U+D7FF, U+E000, U10FFFF 값이다.

fn tuple() {
    let tuple_index: (i32, f64, &str, char) = (100, 3.14, "Tuple", 'a');
    let a = tuple_index.0;
    println!("tuple_index = a : {}", a);

    let b = tuple_index.1;
    println!("tuple_index = b : {}", b);

    let c = tuple_index.2;
    println!("tuple_index = c : {}", c);

    let d = tuple_index.3;
    println!("tuple_index = d : {}", d);
}
// Note : tuple 는 각 다른 데이터 타입을 배열 할 수 있는 방법 중 하나이다.
//        let value: (datatype) = (index)
//        let name = value.index

fn array() {
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("array : {:?}", array);

    let array_index_5 = array[5];
    println!("array_index_5 : {}", array_index_5);
}
// Note : Array 는 하나의 데이터 타입인 값을 배열로 저장하는 방식이다.
//        let value = [index]
//        let name = value[index]
//
//        let value: [type; index] = [index]
//        let name = value[index]