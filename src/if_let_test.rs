pub fn to_test() {
    if let Some(_) = get_some_or_none(false) {
        println!("we found some")
    } else {
        println!("we found none")
    }

    let z = std::option::Option::Some(5);

    z.unwrap_or_else(|| 5);
}

fn get_some_or_none(b: bool) -> Option<i32> {
    if b {
        Some(5)
    } else {
        None
    }
}
