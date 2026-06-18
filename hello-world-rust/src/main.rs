fn greeting() -> String {
    "Hello, world!".to_string()
}

fn main() {
    println!("{}", greeting());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_returns_hello_world() {
        assert_eq!(greeting(), "Hello, world!");
    }

    #[test]
    fn greeting_is_not_empty() {
        assert!(!greeting().is_empty());
    }
}
