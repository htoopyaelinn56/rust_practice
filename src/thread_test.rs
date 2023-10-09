use std::{
    rc::Rc,
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

pub fn thread_test() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn move_test() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

pub fn channel_test() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    //မက်စေ့ချက်ချင်းယူတာ
    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    //does not block main thread but does need loop to check received message
    loop {
        let received = rx.try_recv();
        if let Ok(v) = received {
            println!("received without blocking {v}");
            break;
        }
    }
}

pub fn mutliple_message() {
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
            thread::sleep(Duration::from_millis(1700));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

pub fn mutex_single_thread_test() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

pub fn mutex_multi_thread_test() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            println!("in thread {i} {num}");
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

//only one mutation at a time with Arc
pub fn modifying_state_concurrently() {
    let value: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    let thread_one: JoinHandle<()>;
    let thread_two: JoinHandle<()>;

    let cloned_value_one = Arc::clone(&value);
    let cloned_value_two = Arc::clone(&value);

    //transaction one
    thread_one = thread::spawn(move || {
        let mut v = cloned_value_one.lock().unwrap();
        *v += 1;
        println!("transaction in thread one {}", *v);
    });

    //transaction two
    thread_two = thread::spawn(move || {
        let mut v = cloned_value_two.lock().unwrap();
        *v += 1;
        println!("transaction in thread two {}", *v);
    });

    thread_one.join().unwrap();
    thread_two.join().unwrap();

    println!("Result: {}", *value.lock().unwrap());
}

//doing heavy task at other thread
pub fn message_passing() {
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        let mut v: i64 = 0;
        //heavy calculate
        for i in 1..1000 {
            for j in 1..1000 {
                v += i * j;
            }
        }

        tx.send(v.to_string())
    });

    let result = rx.recv().unwrap();

    println!("Received {}", result);
}
