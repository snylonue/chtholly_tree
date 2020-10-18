use sugars::cell;
use std::collections::BTreeMap;
use std::iter::FromIterator;
use std::cell::Cell;

#[derive(Default, Clone, Copy, Debug)]
pub struct Range {
    pub l: usize,
    pub r: usize,
}
#[derive(Debug)]
pub struct ChthollyTree<T> {
    inner: BTreeMap<Cell<Range>, T>,
    len: usize,
}

impl Range {
    pub const fn new(l: usize, r: usize) -> Self {
        Self { l, r }
    }
}
impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.l.eq(&other.l)
    }
}
impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.l.partial_cmp(&other.l)
    }
}
impl Eq for Range {}
impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.l.cmp(&other.l)
    }
}
impl<T> ChthollyTree<T> {
    pub fn new() -> Self {
        Self { inner: Default::default(), len: 0 }
    }
    pub const fn len(&self) -> usize {
        self.len
    }
}
impl<T: Eq> ChthollyTree<T> {
    pub fn push(&mut self, value: T) {
        match self.inner.iter().next_back() {
            Some((range, v)) if v == &value => {
                let rg = unsafe { &*range.as_ptr() };
                range.set(Range::new(rg.l, rg.r + 1));
            }
            _ => { self.inner.insert(cell!(Range::new(self.len, self.len)), value); }
        }
        self.len += 1;
    }
}
impl<T: Clone> ChthollyTree<T> {
    pub fn split(&mut self, at: usize) {
        let (target, value) = self.inner.range(..=cell!(Range::new(at, 0))).next_back().unwrap();
        let Range { l, r } = target.get();
        if at != r {
            let value = value.to_owned();
            target.set(Range::new(l, at - 1));
            self.inner.insert(cell!(Range::new(at, r)), value);
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