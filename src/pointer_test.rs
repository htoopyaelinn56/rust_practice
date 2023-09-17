pub fn test() {
    let mut a_num = 0;
    inner(&mut a_num);
}

fn inner(x: &mut i32) {
    let another_num = 1;
    let a_stack_ref = &another_num;

    let a_box = Box::new(2);
    let a_box_stack_ref = &a_box;
    let a_box_heap_ref = &*a_box;

    *x += 5;
}

fn ptr_test() {
    let mut s = String::from("Hello");
    let s_ref = &s;
}

fn greet(g1: &String, g2: &String) {
    println!("{} {}!", g1, g2);
}

fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}
