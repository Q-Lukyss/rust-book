use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hello nombre {i} depuis le thread spawned");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap(); // force le thread spawn a fini

    for i in 1..5 {
        println!("Hello nombre {i} du thread principal");
        thread::sleep(Duration::from_millis(1));
    };

    handle.join().unwrap();
}
