use std::collections::BTreeMap;
use std::iter::FromIterator;

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
