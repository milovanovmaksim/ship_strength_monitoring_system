#[cfg(test)]
mod tests {
    use crate::core::binary_search::BinarySearch;

    #[test]
    fn custom_binary_search_test() {
        let data = vec![0.0, 1.1, 1.3, 1.6, 1.8, 2.1, 2.4, 2.8, 5.0];
        for i in 0..data.len() {
            assert_eq!(
                (Some(i), None),
                data.custom_binary_search(*data.get(i).unwrap())
            );
        }
        assert_eq!((None, None), data.custom_binary_search(-1.0));
        assert_eq!((None, None), data.custom_binary_search(6.0));
        assert_eq!((Some(0), Some(1)), data.custom_binary_search(1.0));
        assert_eq!((Some(1), Some(2)), data.custom_binary_search(1.2));
        assert_eq!((Some(2), Some(3)), data.custom_binary_search(1.4));
        assert_eq!((Some(3), Some(4)), data.custom_binary_search(1.7));
        assert_eq!((Some(4), Some(5)), data.custom_binary_search(2.0));
        assert_eq!((Some(6), Some(7)), data.custom_binary_search(2.5));
        assert_eq!((Some(7), Some(8)), data.custom_binary_search(3.0));
    }
}
