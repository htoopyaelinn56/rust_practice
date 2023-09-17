fn test() {}

fn longest<'a, 'b>(x: &'a str, y: &'a str, z: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else if x == y {
        z
    } else {
        y
    }
}
