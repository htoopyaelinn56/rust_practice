use self::List::{Cons, Nil};
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::{ops::Deref, rc::Rc};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("dropping at the end")
    }
}

pub fn test() {
    {
        let x = 5;
        let y = MyBox::new(x);

        drop(y);

        println!("end of inner scope")
    }

    println!("end of main")
}

pub fn ref_counting_test() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    let value_one = Box::new(10);
}

pub fn borrow_test() {
    let mut x = 5;
    let y = &mut x;
}
