use std::collections::BTreeMap;
use std::iter::FromIterator;
use std::ops::RangeBounds;

use num_traits::{Num, NumCast};

#[derive(Debug, Default)]
pub struct ChthollyTree<T> {
    inner: BTreeMap<usize, (usize, T)>,
    len: usize,
}

impl<T> ChthollyTree<T> {
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
            len: 0,
        }
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            cur: None,
            iter: self.inner.iter(),
        }
    }

    pub fn map(&mut self, f: impl Fn(&mut T)) {
        self.inner.iter_mut().for_each(|(_, (_, val))| f(val));
    }

    pub fn fold<Acc>(&self, init: Acc, f: impl Fn(Acc, usize, &T) -> Acc) -> Acc {
        self.inner
            .iter()
            .fold(init, |acc, (l, (r, val))| f(acc, r - l, val))
    }
}

impl<T: Num + NumCast + Clone> ChthollyTree<T> {
    pub fn sum(&self) -> T {
        self.fold(T::zero(), |acc, repeat, val| {
            acc + T::from(repeat).unwrap() * val.clone()
        })
    }

    pub fn range_sum(&mut self, range: impl RangeBounds<usize>) -> T {
        self.fold_range(
            T::zero(),
            |acc, repeat, val| acc + T::from(repeat).unwrap() * val.clone(),
            range,
        )
    }
}

impl<T: Eq> ChthollyTree<T> {
    pub fn push(&mut self, value: T) {
        match self.inner.last_entry() {
            Some(mut entry) if entry.get().1 == value => {
                entry.get_mut().0 += 1;
            }
            _ => {
                self.inner.insert(self.len, (self.len + 1, value));
            }
        }
        self.len += 1;
    }
}

impl<T: Clone> ChthollyTree<T> {
    /// splits a node [l, r) into [l, at) and [at, r)
    /// # Panic
    ///
    /// panic if `at` >= `len`
    pub fn split(&mut self, at: usize) {
        debug_assert!(at < self.len());
        if at == 0 {
            return;
        }

        let (_, (r, val)) = self.inner.range_mut(..=at).next_back().unwrap();
        let rb = *r;

        if at != rb {
            let value = val.clone();
            *r = at;
            self.inner.insert(at, (rb, value));
        }
    }

    fn split_range(&mut self, range: impl RangeBounds<usize>) -> Option<(usize, usize)> {
        let l = match range.start_bound() {
            std::ops::Bound::Included(&l) => l,
            std::ops::Bound::Excluded(l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            std::ops::Bound::Included(r) => match r.checked_sub(1) {
                Some(r) => r,
                None => return None,
            },
            std::ops::Bound::Excluded(&r) => r,
            std::ops::Bound::Unbounded => self.len(),
        };

        if l >= r || r > self.len() {
            return None;
        }

        self.split(l);
        self.split(r);

        Some((l, r))
    }

    pub fn assign(&mut self, val: T, range: impl RangeBounds<usize>) {
        let (l, r) = match self.split_range(range) {
            Some(rg) => rg,
            _ => return,
        };

        self.inner
            .range(l + 1..r)
            .map(|(k, _)| *k)
            .collect::<Vec<_>>()
            .iter()
            .for_each(|k| {
                self.inner.remove(k);
            });

        self.inner.insert(l, (r, val));
    }

    pub fn map_range(&mut self, f: impl Fn(&mut T), range: impl RangeBounds<usize>) {
        let (l, r) = match self.split_range(range) {
            Some(rg) => rg,
            _ => return,
        };

        self.inner.range_mut(l..r).for_each(|(_, (_, val))| f(val));
    }

    pub fn fold_range<Acc>(
        &mut self,
        init: Acc,
        f: impl Fn(Acc, usize, &T) -> Acc,
        range: impl RangeBounds<usize>,
    ) -> Acc {
        let (l, r) = match self.split_range(range) {
            Some(rg) => rg,
            _ => return init,
        };

        self.inner
            .range(l..r)
            .fold(init, |acc, (l, (r, val))| f(acc, r - l, val))
    }
}

impl<T: Eq> FromIterator<T> for ChthollyTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree = Self::new();
        for i in iter {
            tree.push(i);
        }
        tree
    }
}

pub struct Iter<'a, T> {
    cur: Option<(usize, &'a T)>,
    iter: std::collections::btree_map::Iter<'a, usize, (usize, T)>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = match self.cur.as_mut() {
            None | Some((0, _)) => self
                .cur
                .insert(self.iter.next().map(|(l, (r, val))| (r - l, val))?),
            Some(cur) => cur,
        };
        cur.0 -= 1;
        Some(cur.1)
    }
}

#[cfg(test)]
mod test {
    use crate::ChthollyTree;

    #[test]
    fn from_iter() {
        let tree = [1, 1, 2, 3, 4, 5, 5, 5, 5]
            .into_iter()
            .collect::<ChthollyTree<_>>();
        assert_eq!(
            tree.inner.into_iter().collect::<Vec<_>>(),
            vec![
                (0, (2, 1)),
                (2, (3, 2)),
                (3, (4, 3)),
                (4, (5, 4)),
                (5, (9, 5))
            ]
        );
    }

    #[test]
    fn iter() {
        let data = [-1, 2, 2, 3, 0, 0, 0, -4, -4, 10, 10, 12];
        let tree = ChthollyTree::from_iter(data);
        assert_eq!(tree.iter().copied().collect::<Vec<_>>(), data);
    }

    #[test]
    fn split_range() {
        let mut tree = [1, 1, 2, 3, 4, 5, 5, 5, 5]
            .into_iter()
            .collect::<ChthollyTree<_>>();
        tree.split_range(6..8);
        assert_eq!(
            tree.inner.into_iter().collect::<Vec<_>>(),
            vec![
                (0, (2, 1)),
                (2, (3, 2)),
                (3, (4, 3)),
                (4, (5, 4)),
                (5, (6, 5)),
                (6, (8, 5)),
                (8, (9, 5))
            ]
        );
    }

    #[test]
    fn assign() {
        let mut tree = ChthollyTree::from_iter([1, 1, 2, 3, 4, 4, 4, 5, 7, 8]);
        tree.assign(10, 3..6);
        assert_eq!(
            tree.iter().copied().collect::<Vec<_>>(),
            [1, 1, 2, 10, 10, 10, 4, 5, 7, 8]
        );
    }

    #[test]
    fn sum() {
        let tree = ChthollyTree::from_iter([1, 1, 2, 3, 4, 4, 4, 5, 7, 8]);
        assert_eq!(tree.sum(), [1, 1, 2, 3, 4, 4, 4, 5, 7, 8].into_iter().sum());
    }

    #[test]
    fn range_fold() {
        let mut tree = ChthollyTree::from_iter([1, 1, 2, 3, 4, 4, 4, 5, 7, 8]);
        assert_eq!(
            tree.range_sum(3..6),
            [1, 1, 2, 3, 4, 4, 4, 5, 7, 8][3..6].into_iter().sum()
        );
    }
}
