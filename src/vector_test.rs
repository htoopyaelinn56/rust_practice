pub fn test() {
    let mut v: Vec<i32> = vec![1, 2];

    v.append(&mut v.clone());

    println!("{:?}", v);
    v.push(3);
    println!("{:?}", v);
}
