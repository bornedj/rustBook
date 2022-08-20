use std::thread; 
use std::time::Duration;
pub fn intro_concurrency() {
    // thread will not finish it's execution before the main thread and as such will not finish its execution
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread.", i);
            thread::sleep(Duration::from_micros(1));// forces the thread to stop execution for a set duration, which allows another thread to run
        }
    });

    for i in 1..5 {
        println!("hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

}

pub fn join_handle () {
    // thread will finish it's execution as it's value is saved to a variable 
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread.", i);
            thread::sleep(Duration::from_micros(1));// forces the thread to stop execution for a set duration, which allows another thread to run
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

}

pub fn move_example () {
    // using move to force a thread's closure to take ownership of v
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
