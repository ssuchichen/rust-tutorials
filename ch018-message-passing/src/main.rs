use std::sync::mpsc;
use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    let (transmitter, receiver) = mpsc::channel::<u8>();
    // 关闭
    // drop(receiver);

    // 不会阻塞
    let _ = receiver.recv_timeout(Duration::from_millis(200));

    //发送器和接收器任何一个关闭，整个通道会失效，都会发送/接收失败
    // let send_result = transmitter.send(88);
    // println!("Send status: {}", send_result.is_ok());
    //
    // let _ = transmitter.send(168);

    // 没有收到值会阻塞
    //receiver.recv();

    // let receive_result = receiver.recv_timeout(Duration::from_millis(200));
    // println!("Receive status is {}", receive_result.is_ok());
    // println!("Receive result is {}", receive_result.unwrap());
    //
    // let receive_result = receiver.recv_timeout(Duration::from_millis(200));
    // println!("Receive status is {}", receive_result.is_ok());
    // println!("Receive result is {}", receive_result.unwrap());

    let process = move || {
        println!("Starting processor thread ...");
        let mut failed_count = 0u8;
        loop {
            println!("Attempting to receive a message from channel ...");
            let receive_result = receiver.recv_timeout(Duration::from_millis(800));
            if receive_result.is_ok() {
                println!("Received message: {}", receive_result.unwrap());
            } else {
                failed_count += 1;
            }
            if failed_count > 10 {
                println!("Aborting processor thread ... no work available");
                break;
            }
        }
    };

    for x in 1..=10u8 {
        let send_result = transmitter.send(x);
        println!("Send status: {}", send_result.is_ok());
        sleep(Duration::from_millis(200));
    }

    let _ = spawn(process).join();
}
