pub fn add(a: i32, b: i32) -> i32 {
    /// Add two signed integers.
    return a + b;
}

#[cfg(test)]
mod test {

    use memoryBank;

    #[test]
    fn memoryBankTest() {
        assert_eq!(memoryBank::add(1, 1), 2);
    }

    fn memoryBankTest() {
        assert_eq!(memoryBank::add(1, 1), 2);
    }
}
