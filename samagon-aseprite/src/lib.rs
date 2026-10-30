pub fn add_2(left: u64, right: u64) -> u64 {
    left + right + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_2(2, 2);
        assert_eq!(result, 6);
    }
}
