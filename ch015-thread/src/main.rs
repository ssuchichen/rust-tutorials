use std::thread::spawn;

pub fn test_thread() {
    let mut x = 0u128;
    for i in 1..=500 {
        x += i;
    }
    println!("\x1b[38;2;100;100;255mMain thread finished a little bit of work.\x1b[0m");
}

fn main() {
    let thread_fn = || {
        let mut x = 0u128;
        for i in 1..=50_000_000 {
            x += i;
        }
    };

    let handle = spawn(thread_fn);
    let handle2 = spawn(thread_fn);

    // test_thread();
    // handle.join();
    // handle2.join();

    loop {
        test_thread();
        if handle.is_finished() && handle2.is_finished() {
            println!("All the workers are Done.");
            break;
        }
    }
}
