use std::thread::{scope, spawn};

struct Person {
    first_name: String,
}

fn main() {
    let age = 30;
    let person01 = Person {
        first_name: String::from("Mark"),
    };

    // 不需要move关键字
    let print_num = || {
        println!("Your age is {age}");
        println!("Your name is {}", &person01.first_name);
    };

    // let _ = spawn(print_num).join();

    // 与直接使用spawn不同，scope允许你在子线程中引用外部作用域的数据，而不需要将数据的所有权转移给线程
    // scope创建了一个新的作用域，在这个作用域内启动的任何线程都会在作用域结束前完成。
    // 这意味着你不需要显式地调用join()来等待每个线程完成，scope会自动为我们做这件事。
    // 此外，由于作用域的存在，我们可以安全地从作用域内的线程访问外部作用域的变量，而无需担心生命周期的问题。
    scope(|s| {
        s.spawn(print_num);
        s.spawn(print_num);
        s.spawn(print_num);
    }); //所有在scope内启动的线程在此处自动join
    // 在这里所有线程已经完成。
    println!("Giving control back to main thread.");
    println!("Your name is {}", person01.first_name);

    println!("Finished printing age");
}
