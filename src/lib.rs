pub fn randnum() -> u8 {
    45
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_randnum() {
        assert_eq!(randnum(), 42);
    }
}
