use std::cell::RefCell;

pub fn test() {
    let mut owner = 5;

    let b_two = &mut owner;
    *b_two = 3;

    let ref_owner = RefCell::new(5);
    let ref_borrower = ref_owner.borrow_mut();

    ref_borrower;

    println!("{:?}", ref_owner)
}
