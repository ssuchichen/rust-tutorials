use futures::{FutureExt, pin_mut, select};
use smol::block_on;
use std::thread::sleep;
use std::time::Duration;

async fn get_number1() -> u16 {
    println!("Running get_number1");
    168
}

async fn get_number2() -> u16 {
    sleep(Duration::from_millis(50));
    println!("Running get_number2");
    666
}

async fn get_number3() -> u16 {
    sleep(Duration::from_millis(75));
    println!("Running get_number3");
    888
}

fn main() {
    let num1 = get_number1().fuse();
    let num2 = get_number2().fuse();
    let num3 = get_number3().fuse();

    pin_mut!(num1, num2, num3);

    let result = block_on(async {
        // join!(num3, num2, num1)
        loop {
            select! {
                x = num1 => println!("num1 is completed {}", x),
                x = num2 => println!("num2 is completed {}", x),
                x = num3 => println!("num3 is completed {}", x),
                complete => {
                    println!("All futures hava finished polling. Breaking out of loop!");
                    break;
                }
            }
        }
    });

    println!("Final value is {:?}", result);
}
