#[cfg(test)]
mod tests {

    #[test]
    fn exploration() {
        // assert macro evaluates a boolean, if it's true the test will be ok
        // if it's false assert will call panic and test will failed
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Panic makes test failed");
    }
}