pub fn add_two(number: i32) -> i32 {
    number + 2
}

#[cfg(test)]
mod tests {

    #[test]
    fn exploration() {
        // assert macro evaluates a boolean, if it's true the test will be ok
        // if it's false assert will call panic and test will failed
        assert_eq!(2 + 2, 4);
    }

    // If at least a unit test failed, integration test won't run, so that's why
    // I added the ignore to allow the code run the integration test
    #[test]
    #[ignore]
    fn another() {
        panic!("Panic makes test failed");
    }
}