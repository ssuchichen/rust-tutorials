use std::ops::AddAssign;
use std::sync::Mutex;
use std::thread::{scope, sleep};
use std::time::Duration;

fn main() {
    let score = Mutex::new(0u16);
    // let guard = score.lock();
    // let mut data = guard.unwrap();
    // data.add_assign(5);
    // println!("{:?}", data);
    // drop(data);

    let my_fn = || {
        println!("Thread 1 is waiting for mutex lock...");
        let mut data = score.lock().unwrap();
        for i in 1..10 {
            data.add_assign(i);
            println!("Thread 1 is adding {i}");
            sleep(Duration::from_millis(400));
        }
    };

    let my_fn2 = || {
        loop {
            println!("Thread 2 is waiting for mutex lock...");
            let guard = score.try_lock();
            if guard.is_ok() {
                let mut data = guard.unwrap();
                for i in 1..10 {
                    data.add_assign(i);
                    println!("Thread 2 is adding {i}");
                }
                break;
            }
            sleep(Duration::from_millis(200));
        }
    };

    _ = scope(|s| {
        s.spawn(my_fn);
        s.spawn(my_fn2);
    });

    println!("{:?}", score.lock().unwrap());
}
