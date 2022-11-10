use std::thread;
use std::time::Duration;

pub fn run(){
    thread::spawn(|| {
        for i in 0..10 {
            println!("This is spwan thread printing {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 0..10 {
        println!("This is main thread printing {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}