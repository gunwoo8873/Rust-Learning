use std::thread::{  // std : 표준 내장 라이브러리 [모든 Thread에 고유 식별자를 할당]
    spawn,          // spawn : 함수 이름을 전달할 수 있지만 Closure를 전달할 수 있는 Module이다.
    current,        // current : 현재 수행중인 Thread에 대한 Handle을 반환하는 Module이다.
    scope,          // scope : 특정 Scope 에서만 존재하면 해당 범위에서 존재하는 지역 변수의 소유권을 빌려올 수 있다.
};

// Rc, Arc 는 Reference Counting을 통해 데이터 소유권을 어려 범위에서 공유할 수 있도록 하는 Pointer다.
use std::rc::{Rc};  // Rc  : 단일 Thread 환경 한정으로 Atomic 연산을 사용하지 않아 속도는 Arc보다 빠르다.
use std::sync::Arc; // Arc : Multi Thread 환경 한정으로 Atomic 연산을 사용하지만 속도는 Rc보다 느리다.

fn main()
{
    j1(); // Thread [2]
    j2(); // Thread [3]
    println!("End Thread Function"); // println는 io::Stdou::lock()을 사용해 다른 Thread로 부터 간섭받지 않도록 한다.

    lock_thread();
    scope_thread();
    counting_thread();
    arc_thread();
}

fn j1()
{
    let a = spawn(j_f);
    println!("J1 Thread");

    a.join().unwrap();

    // Noti : spawn 함수에서 리턴된 joinHandle을 사용하며 작업이 끝날 때까지 main 대기한다.
    //        join() : thread::Result를 리턴한다 Result는 에러가 발생하면 Panic의 MSG 를 포함하여 출력한다.
    //        unwrap() : 해당 Thread의 작업이 종료될 때 Panic을 발생 시킬수 있다.
}

fn j2()
{
    let a = spawn(j_f);
    println!("J2 Thread");

    a.join().unwrap(); // J1의 코드들이 수행이 끝날 때 까지 대기한다.
}

fn j_f()
{
    println!("Another thread");

    let id = current().id(); // Type : ThreadId
    println!("Thread id : {:?}", id) // Return Thread ID Output
}

fn lock_thread()
{
    let a = vec![1, 2, 3];
    spawn(move || { // move를 사용하지 않는다면 Closure는 Reference로 캡처[Capture] 하게되며 컴파일 에러가 발생한다.
            for n in a
            {
                println!("lock : {}", n);
            }
        }
    )
        .join().unwrap(); // join Method가 호출될 때 Result로 감싸져 리턴된다.


    let b = Vec::from_iter(0..=1000);
    let b_thread = spawn(move || {
            let len = b.len();
            let sum = b.into_iter().sum::<usize>();
            sum / len
        }
    );

    // Note : thread::spawn은 thread::Builder::new().spawn().unwrap()를 대체하여 간편하기 사용 위한 형태다.
    //        Builder는 여러 옵션 설정이 가능하며 Stack Memory Size와 Label을 붙일 수 있다.
    //        label name 확인은 current().name() 통해 확인이 가능하다. [Panic MSG에 포함되어 Monitoring Tool에서 확인 할 수 있다.

    let average = b_thread.join().unwrap();
    println!("Average Thread Value: {}", average);
}

fn scope_thread()
{
    let a = vec![1, 2, 3];
    scope(|s| { // Scope에 Closuref를 전달 인수는 s로 설정하여 입력을 받는다.
            s.spawn(|| {
                println!("Thread Len : {}", a.len());
            });

            s.spawn(|| {
                for n in &a
                {
                    println!("lock : {}", n);
                }
            });
        }
    );

    static B: [i32; 5] = [1, 2, 3, 4, 5]; // static으로 선언된 변수는 initalizer를 갖고 드랍되지 않은 상태에서 생성된다.
    spawn(|| dbg!(&B)); // dbg! : Debug Output


    let c: &'static [i32; 5] = Box::leak(Box::new([1, 2, 3, 4, 5]));
    spawn(move || dbg!(c));

    // Waring : 강제로 누수[leaking] 발생 시키는 방법이니 실제 적용은 권장하지 않는다.
    //          또한 Box는 별도의 use를 사용하지 않고 코드 범위 안에서 사용하는 것 같다.
    //          'static의 라이프타임은 프로그램이 종료될 때 까지 유지되어 메모리의 누수가 발생한다.
}

fn counting_thread()
{
    let a = Rc::new([1, 2, 3]);
    let b = a.clone();       // a = [1,2,3]을 복사

    assert_eq!(a.as_ptr(), b.as_ptr()); // a와 b의 raw pointer를 같은지 비교하고, 같은 메모리를 가리킨다.

    // Note : as_ptr는 if의 == 연산자와 비슷한 성격을 가지고 있다. 같은 위치를 가리키고 있는지 확인하여,
    //        메모리 주소를 가리키는 raw pointer를 얻는다. [Waring : 메모리의 안전성을 보장받지 못하는 코드다. = unsafe]
    //        assert_eq!는 두 개의 값이 같은지 확인하여, 수행을 이어서 하지만 다르다면 즉시 실행을 중단하고
    //        Panic이 발생한다. [즉 오류가 발생하여 오류에 대한 MSG를 출력]
}

fn arc_thread()
{
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();

    spawn(move || dbg!{a});
    spawn(move || dbg!(b));
}