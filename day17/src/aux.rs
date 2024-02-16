pub mod aux {
    use std::array::from_fn;


    /// Check whether an n-dimensional index is within bounds of an n-dimensional shape.
    /// 
    /// Returns `Some` with the index as a `Box<[usize]>` if the index is within bounds, and `None` otherwise.
    /// 
    /// # Example
    /// 
    /// ```
    /// use aoc::aux::within_bounds;
    /// 
    /// let shape = [3, 4, 5];
    /// assert_eq!(within_bounds(&[0, 0, 0], &shape), Some(Box::new([0, 0, 0])));
    /// assert_eq!(within_bounds(&[2, 3, 4], &shape), Some(Box::new([2, 3, 4])));
    /// assert_eq!(within_bounds(&[3, 0, 0], &shape), None);
    /// assert_eq!(within_bounds(&[1, -1, 0], &shape), None);
    /// ```
    pub fn within_bounds<const DIM: usize>(index: [isize; DIM], shape: [usize; DIM]) -> Option<[usize; DIM]> {
        let index_unsigned: [usize; DIM] = from_fn(|i| index[i] as usize);
        for (x, s) in index.into_iter().zip(shape.into_iter()) {
            if x < 0 || x as usize >= s {
                return None;
            }
        }
        Some(index_unsigned.into())
    }
}
