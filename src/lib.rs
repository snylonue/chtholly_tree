use std::collections::BTreeMap;
use std::iter::FromIterator;

#[derive(Default, Clone, Copy)]
pub struct Range {
    pub l: usize,
    pub r: usize,
}
pub struct ChthollyTree<T> {
    inner: BTreeMap<Range, T>,
    len: usize,
}

impl Range {
    pub fn new(l: usize, r: usize) -> Self {
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
        Self { inner: BTreeMap::new(), len: 0 }
    }
    pub const fn len(&self) -> usize {
        self.len
    }
}
impl<T: Clone> ChthollyTree<T> {
    pub fn split(&mut self, at: usize) {
        let (range, v) = self.inner.remove_entry(&Range::new(at, 0)).unwrap();
        self.inner.insert(Range::new(range.l, at), v.clone());
        self.inner.insert(Range::new(at + 1, range.r), v);
    }
}
impl<T: Eq> FromIterator<T> for ChthollyTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree = BTreeMap::new();
        // let mut iter = iter.into_iter();
        let mut len = 0usize;
        let mut range = Range::default();
        let mut last = None;
        for el in iter {
            len += 1;
            match last {
                Some(ref ls) if ls == &el => range.r += 1,
                Some(ls) => {
                    tree.insert(range, ls);
                    range.r += 1;
                    range.l = range.r;
                    last = Some(el);
                }
                None => { last.replace(el); }
            };
        }
        if let Some(el) = last {
            tree.insert(range, el);
        }
        Self { inner: tree, len }
    }
}