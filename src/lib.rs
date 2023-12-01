use std::collections::BTreeMap;
use std::iter::FromIterator;
use std::ops::RangeBounds;

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
        let r = match range.start_bound() {
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
        self.split(r - 1);

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
