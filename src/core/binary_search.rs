pub(crate) trait BinarySearch {
    fn custom_binary_search(&self, value: f64) -> (Option<usize>, Option<usize>);
}


impl BinarySearch for Vec<f64> {

    ///
    /// Бинарный поиск. Вектор должен быть отсортирован по возрастанию иначе результат бессмысленен.
    /// Возвращает индекс найденного значения в векторе ```(Some(index), None)```.
    /// Если значение в векторе не неайдено, возвращает индексы ближайших значений между которыми лежит
    /// заданное значение ```(Some(index), Some(index))```.
    /// Если значение выходит за максимальную или минимальную границу вектора, возвращает ```(None, None)```.
    ///
    /// # Examples
    /// ```
    /// let data = vec![0.0, 1.1, 1.3, 1.6, 1.8, 2.1, 2.4, 2.8, 5.0];
    /// for i in 0..data.len() {
    ///     assert_eq!((Some(i), None), data.custom_binary_search(*data.get(i).unwrap()));
    /// }
    /// assert_eq!((None, None), data.custom_binary_search(-1.0));
    /// assert_eq!((None, None), data.custom_binary_search(6.0));
    /// assert_eq!((Some(0), Some(1)), data.custom_binary_search(1.0));
    /// assert_eq!((Some(1), Some(2)), data.custom_binary_search(1.2));
    /// assert_eq!((Some(2), Some(3)), data.custom_binary_search(1.4));
    /// assert_eq!((Some(3), Some(4)), data.custom_binary_search(1.7));
    /// assert_eq!((Some(4), Some(5)), data.custom_binary_search(2.0));
    /// assert_eq!((Some(6), Some(7)), data.custom_binary_search(2.5));
    /// assert_eq!((Some(7), Some(8)), data.custom_binary_search(3.0));
    /// ```
    fn custom_binary_search(&self, value: f64) -> (Option<usize>, Option<usize>)  {
        if self.len() == 0 {
            return (None, None)
        }
        let mut left_point = 0;
        let mut right_point = self.len() - 1;
        if *self.first().unwrap() > value || value > *self.last().unwrap() {
            return (None, None);
        }
        while left_point != right_point - 1 {
            let middle = (left_point + right_point) / 2;
            let searched_draft = *self.get(middle).unwrap();
            if searched_draft == value {
                return (Some(middle), None);
            } else if searched_draft > value {
                right_point = middle;
            } else { left_point = middle }
        }
        if *self.get(left_point).unwrap() == value {
            return (Some(left_point), None);
        }
        if *self.get(right_point).unwrap() == value {
            return (Some(right_point), None);
        }
        (Some(left_point), Some(right_point))
    }
}
