pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(feature = "foo")]
pub fn foo() -> u64 {
    1 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
