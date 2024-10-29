use std::thread::{  // std : 표준 내장 라이브러리 [모든 Thread에 고유 식별자를 할당]
    spawn,          // spawn : 함수 이름을 전달할 수 있지만 Closure를 전달할 수 있는 Module이다.
    current,        // current : 현재 수행중인 Thread에 대한 Handle을 반환하는 Module이다.
};

fn main()
{
    j1(); // Thread [2]
    j2(); // Thread [3]
    println!("End Thread Function"); // println는 io::Stdou::lock()을 사용해 다른 Thread로 부터 간섭받지 않도록 한다.

    lock();
}

fn j1()
{
    let a = spawn(f);
    println!("J1 Thread");

    a.join().unwrap();
    /*
    spawn 함수에서 리턴된 joinHandle을 사용하며 작업이 끝날 때까지 main 대기한다.
    join() : thread::Result를 리턴한다 Result는 에러가 발생하면 Panic의 MSG 를 포함하여 출력한다.
    unwrap() : 해당 Thread의 작업이 종료될 때 Panic을 발생 시킬수 있다.
    */
}

fn j2()
{
    let a = spawn(f);
    println!("J2 Thread");

    a.join().unwrap(); // J1의 코드들이 수행이 끝날 때 까지 대기한다.
}

fn f()
{
    println!("Another thread");

    let id = current().id(); // Type : ThreadId
    println!("Thread id : {:?}", id) // Return Thread ID Output
}

fn lock()
{
    let num = vec![1, 2, 3];
    spawn(move || {
        for n in num {
            println!("lock : {}", n);
        }
    })
        .join().unwrap();
}