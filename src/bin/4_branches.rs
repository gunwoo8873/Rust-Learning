pub fn main() {
    if_branches();
    loop_branches();
    label_loop_branches();
    while_branches();
    for_branches();
}

fn if_branches() {
    let a = 60;
    if a > 50 {
        println!("true");
    }
    else if a < 30 {
        println!("false");
    }
}

fn loop_branches() {
    let mut default_count = 0;

    //// Not Label Loop
    let result = loop {
        // println!("loop"); // Waring : infinite loop
        default_count += 1;

        if default_count == 10 {
            break default_count; // Return Value
        }
    };

    println!("Loop Result : {result}");
}
// Note : let 을 통한 Loop 는 default_count 가 0에서 10에 도달할 때 까지의 과정을 출력하지 않고 결과값인 10만 출력한다.

fn label_loop_branches() {
    let mut default_count = 0;

    //// Label Loop
    'counting_up: loop { // 'name : loop {...} = loop label?
        println!("count = {default_count}");

        let mut remain = 10;

        loop {
            println!("remain = {remain}");

            if remain == 9 {
                break;
            }

            if default_count == 5 {
                break 'counting_up; // Break after Return Label loop run
            }

            remain -= 1;
        }

        default_count += 1;
    }

    println!("End Count = {default_count}");
}
// Note : 다중 Loop 에서 remain 이 10에서 9로 반환값을 출력을 할 때 default_count 가 0에서 5에 도달 할 때 까지 반복을 한다.
//        하지만 '의 사용처는 많아 단순히 특정 Loop 의 이용함이 아니라 문자열이나 타입 명시 할 경우에 사용하는 것 같다.

fn while_branches() {
    let mut default_count = 0;

    while default_count < 5 {
        default_count += 1;

        if default_count == 5 {
            break
        }
        println!("While End Result : {}", default_count);
    }

    let arr = ["A", "B", "C", "D", "E"];

    while default_count < 5 {
        println!("Array While End Result = {}", arr[default_count]); // arr[index] = name[index 0..5]
        default_count += 1;
    }
}

fn for_branches() {
    let arr = [10, 20, 30, 40, 50];

    for element in arr { // in arr = arr[index] bind
        println!("{}", element);
    }

    for element in (1..5).rev() { // in (1..5) = 1 ~ 4
        println!("{}", element);
    }
}
// Note : for 은 컬렌션의 아이템에 대하여 임의의 코드를 수행시킬 수 있다.