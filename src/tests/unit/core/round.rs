#[cfg(test)]
mod tests {
    use crate::core::round::Round;

    #[test]
    fn test_round() {
        assert_eq!(2.0, 2.0.my_round(1));
        assert_eq!(2.1, 2.08.my_round(1));
        assert_eq!(2.0, 2.04.my_round(1));
        assert_eq!(24.25, 24.2499996.my_round(2));
        assert_eq!(2.1, 2.05.my_round(1));
        assert_eq!(3.0, 2.5.my_round(0));
        assert_eq!(20.83, 20.8333.my_round(2))
    }
}
