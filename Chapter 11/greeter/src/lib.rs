pub fn greeting(name: &str) -> String {
    //format!("Hello {}!", name)
    format!("Hello")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        //assert!(result.contains("Carol"));
        assert!(
            result.contains("Carol"),
            "Greeting didn't contain name, it was {}", result
        );
    }
}