use std::thread; 
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc};

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

pub fn channel_example () {
    let (tx, rx) = mpsc::channel();
    // sending a string through our channel to satisfy the channel declaration
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    // receiving the string from the thread that took ownership of the transmitter
    let received = rx.recv().unwrap();// recv blocks the main threads execution and waits for the message to be sent downstream
    println!("Got: {}", received);


}

pub fn channel_display_concurrency () {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // loop waiting to receive values from the spawned thread
    for received in rx {
        println!("Got: {}", received);
    }
}

// demonstrates how multiple channels can pass values to a single consumer
pub fn multiple_producer_single_consumer () {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

}

pub fn example_mutex () {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();// lock blocks the thread from doing work until we have ownership of the lock
        *num = 6;
    }

    println!("m = {:?}", m);
}

pub fn sharing_mutex_btw_threads () {
    let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
