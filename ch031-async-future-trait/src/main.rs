use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use tokio::task;

async fn test_something() {
    sleep(Duration::from_millis(5000));
    println!("Hello from Tokio!");
}

struct F1Racer {
    name: String,
    completed_laps: u8,
    laps: u8,
    best_lap_time: u8,
    lap_times: Vec<u8>,
}

impl F1Racer {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            completed_laps: 0,
            laps: 5,
            best_lap_time: 255,
            lap_times: vec![87, 64, 126, 95, 76],
        }
    }

    fn do_lap(&mut self) {
        println!("{} is doing a new lap...", self.name);
        let lap_time = self.lap_times.pop();
        if lap_time.is_some() && lap_time.unwrap() < self.best_lap_time {
            self.best_lap_time = lap_time.unwrap();
        }
        self.completed_laps += 1;
    }
}

impl Future for F1Racer {
    type Output = u8;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Thread assigned id is : {:?}", thread::current().id());
        if self.completed_laps < self.laps {
            self.do_lap();
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }
        println!("{} has completed all laps!", self.name);
        println!("Best lap time for is {}", self.best_lap_time);
        Poll::Ready(self.best_lap_time)
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    // test_something().await;
    let racer01 = F1Racer::new("Max Verstappen");
    let mut racer02 = F1Racer::new("Json Smith");

    racer02.lap_times.pop();
    racer02.lap_times.push(57);

    let handle01 = task::spawn(racer01);
    let handle02 = task::spawn(racer02);
    loop {
        if handle01.is_finished() && handle02.is_finished() {
            println!("All racers have finished");
            break;
        }
        sleep(Duration::from_millis(300));
    }
}
