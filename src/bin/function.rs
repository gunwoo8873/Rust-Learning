fn main() {
    parameter_function_a(15, 20.25); // Format value
    parameter_function_b("Parameter", 'B');
    expression_function();

    let x = return_function(5); // Variable Value
    println!("return : {x}");
}

fn parameter_function_a(x: i32, y: f64) {
    println!("parameter = x : {x}, y : {y}");
}

fn parameter_function_b(value: &str, label: char) {
    println!("parameter = value : {label}, label : {value}");
}

/*
함수는 Parameter 를 갖도록 정의가 가능하여 하나의 function signature 의 일부인 함수이다.
Parameter 에 대한 구체적인 값을 전달이 가능하다. 단 데이터 타입에 대한 명시는 반드시 해야한다.
 */


fn expression_function() {
    let x = { // New Scope
        let y = 50.01; // New Value Binding
        y + 1.0

    };

    println!("Expression : {x}"); // x = y + 1.0
}

fn return_function(x: i32) -> i32 {
    x + 6 // Return Value
}