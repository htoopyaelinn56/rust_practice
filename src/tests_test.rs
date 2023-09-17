pub fn to_test() -> i32 {
    50
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = to_test();
        assert_eq!(result, 50);
    }
}
        