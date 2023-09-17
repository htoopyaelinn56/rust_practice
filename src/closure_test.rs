use std::thread;

pub fn test() {
    let add_one_closure = |x: i32| x + 1;

    let one_plus_one = add_one_closure(1);

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut only_borrows = || {
        println!("From closure: {:?}", list);
        list.push(5)
    };

    only_borrows();
    println!("After calling closure: {:?}", list);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
