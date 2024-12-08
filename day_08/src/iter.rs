pub struct Combinations<'a, T> {
    buffer: &'a [T],
    idx: usize,
    pair_idx: usize,
}

impl<'a, T> Iterator for Combinations<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.buffer.get(self.idx)?;
        if let Some(b) = self.buffer.get(self.pair_idx) {
            self.pair_idx += 1;
            Some((a, b))
        } else {
            self.idx += 1;
            self.pair_idx = self.idx + 1;
            self.next()
        }
    }
}

impl<'a, T> From<&'a [T]> for Combinations<'a, T> {
    fn from(value: &'a [T]) -> Self {
        Self {
            buffer: value,
            idx: 0,
            pair_idx: 1,
        }
    }
}
pub struct CombinationsOwned<T> {
    buffer: Vec<T>,
    idx: usize,
    pair_idx: usize,
}

impl<T: Clone> Iterator for CombinationsOwned<T> {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.buffer.get(self.idx)?;
        if let Some(b) = self.buffer.get(self.pair_idx) {
            self.pair_idx += 1;
            Some((a.clone(), b.clone()))
        } else {
            self.idx += 1;
            self.pair_idx = self.idx + 1;
            self.next()
        }
    }
}

impl<T> From<Vec<T>> for CombinationsOwned<T> {
    fn from(value: Vec<T>) -> Self {
        Self {
            buffer: value,
            idx: 0,
            pair_idx: 1,
        }
    }
}

#[test]
fn test_comb() {
    let a = [1, 2, 3, 4];
    let mut iter = Combinations::from(a.as_slice());
    assert_eq!(Some((&1, &2)), iter.next());
    assert_eq!(Some((&1, &3)), iter.next());
    assert_eq!(Some((&1, &4)), iter.next());
    assert_eq!(Some((&2, &3)), iter.next());
    assert_eq!(Some((&2, &4)), iter.next());
    assert_eq!(Some((&3, &4)), iter.next());
    assert_eq!(None, iter.next());
}
