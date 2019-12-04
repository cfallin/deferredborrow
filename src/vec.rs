use crate::DefBorrow;

use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::mem;
use std::ops::{Index, IndexMut};
use std::slice;

pub struct AppendOnlyVec<T, Tag> {
    v: Vec<T>,
    _tag: PhantomData<Tag>,
}

impl<T, Tag> PartialEq for AppendOnlyVec<T, Tag>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.v.eq(&other.v)
    }
}
impl<T, Tag> Eq for AppendOnlyVec<T, Tag> where T: Eq {}
impl<T, Tag> PartialOrd for AppendOnlyVec<T, Tag>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.v.partial_cmp(&other.v)
    }
}
impl<T, Tag> Ord for AppendOnlyVec<T, Tag>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.v.cmp(&other.v)
    }
}
impl<T, Tag> Hash for AppendOnlyVec<T, Tag>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.v.hash(state);
    }
}
impl<T, Tag> fmt::Debug for AppendOnlyVec<T, Tag>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.v.fmt(f)
    }
}

// Note: FrozenVec does NOT implement Clone. Doing so would be unsound because it would create two
// different AppendOnlyVecs with the same tag. A reference to a subsequently appended element on
// one of them could then be used on the other, referring to an arbitrary (or nonexistent) element.

#[derive(Clone, Copy)]
pub struct AppendOnlyVecRef<T, Tag> {
    idx: usize,
    _phantom1: PhantomData<T>,
    _phantom2: PhantomData<Tag>,
}

impl<T, Tag> AppendOnlyVec<T, Tag> {
    pub fn new(v: Vec<T>, _tag: Tag) -> AppendOnlyVec<T, Tag> {
        AppendOnlyVec {
            v,
            _tag: PhantomData,
        }
    }

    pub fn deferred(&self, idx: usize) -> AppendOnlyVecRef<T, Tag> {
        AppendOnlyVecRef {
            idx,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }

    pub fn push(&mut self, t: T) {
        self.v.push(t);
    }

    pub fn to_vec(self) -> Vec<T> {
        self.v
    }

    pub fn iter<'a>(&'a self) -> slice::Iter<'a, T> {
        self.v.iter()
    }
    pub fn iter_mut<'a>(&'a mut self) -> slice::IterMut<'a, T> {
        self.v.iter_mut()
    }

    pub fn elems_deferred(&self) -> Vec<AppendOnlyVecRef<T, Tag>> {
        (0..self.v.len()).map(|i| self.deferred(i)).collect()
    }
}

impl<T, Tag> Index<usize> for AppendOnlyVec<T, Tag> {
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        self.v.index(idx)
    }
}
impl<T, Tag> IndexMut<usize> for AppendOnlyVec<T, Tag> {
    fn index_mut(&mut self, idx: usize) -> &mut T {
        self.v.index_mut(idx)
    }
}

impl<T, Tag> DefBorrow<AppendOnlyVec<T, Tag>, T> for AppendOnlyVecRef<T, Tag> {
    fn def_borrow<'a>(&self, base: &'a AppendOnlyVec<T, Tag>) -> &'a T {
        &base.v[self.idx]
    }

    fn def_borrow_mut<'a>(&self, base: &'a mut AppendOnlyVec<T, Tag>) -> &'a mut T {
        &mut base.v[self.idx]
    }
}

pub struct FrozenVec<T, Tag> {
    v: Vec<T>,
    _tag: PhantomData<Tag>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct FrozenVecRef<T, Tag> {
    ptr: *mut T,
    _tag: PhantomData<Tag>,
}

impl<T, Tag> FrozenVec<T, Tag> {
    pub fn new(v: Vec<T>, _tag: Tag) -> FrozenVec<T, Tag> {
        FrozenVec {
            v,
            _tag: PhantomData,
        }
    }

    pub fn deferred(&self, idx: usize) -> FrozenVecRef<T, Tag> {
        FrozenVecRef {
            ptr: unsafe { mem::transmute(&self.v[idx] as *const T) },
            _tag: PhantomData,
        }
    }

    pub fn to_vec(self) -> Vec<T> {
        self.v
    }

    pub fn iter<'a>(&'a self) -> slice::Iter<'a, T> {
        self.v.iter()
    }
    pub fn iter_mut<'a>(&'a mut self) -> slice::IterMut<'a, T> {
        self.v.iter_mut()
    }

    pub fn elems_deferred(&self) -> Vec<FrozenVecRef<T, Tag>> {
        (0..self.v.len()).map(|i| self.deferred(i)).collect()
    }
}

impl<T, Tag> PartialEq for FrozenVec<T, Tag>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.v.eq(&other.v)
    }
}
impl<T, Tag> Eq for FrozenVec<T, Tag> where T: Eq {}
impl<T, Tag> PartialOrd for FrozenVec<T, Tag>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.v.partial_cmp(&other.v)
    }
}
impl<T, Tag> Ord for FrozenVec<T, Tag>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.v.cmp(&other.v)
    }
}
impl<T, Tag> Hash for FrozenVec<T, Tag>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.v.hash(state);
    }
}
impl<T, Tag> fmt::Debug for FrozenVec<T, Tag>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.v.fmt(f)
    }
}

impl<T, Tag> Index<usize> for FrozenVec<T, Tag> {
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        self.v.index(idx)
    }
}
impl<T, Tag> IndexMut<usize> for FrozenVec<T, Tag> {
    fn index_mut(&mut self, idx: usize) -> &mut T {
        self.v.index_mut(idx)
    }
}

impl<T, Tag> DefBorrow<FrozenVec<T, Tag>, T> for FrozenVecRef<T, Tag> {
    fn def_borrow<'a>(&self, _base: &'a FrozenVec<T, Tag>) -> &'a T {
        unsafe { mem::transmute(self.ptr) }
    }

    fn def_borrow_mut<'a>(&self, _base: &'a mut FrozenVec<T, Tag>) -> &'a mut T {
        unsafe { mem::transmute(self.ptr) }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    fn f<Tag>(v: &AppendOnlyVec<usize, Tag>, ref1: AppendOnlyVecRef<usize, Tag>) -> usize {
        *d!(v, ref1)
    }

    #[test]
    fn test_append_only_vec() {
        let v = vec![1, 2, 3, 4];
        let w = vec![5, 6, 7, 8];

        let mut v = freeze!(AppendOnlyVec, v);
        let mut w = freeze!(AppendOnlyVec, w);

        let ref1 = deferred!(v, 0);
        let ref2 = deferred!(w, 0);

        for i in 0..100 {
            v.push(i);
        }

        println!("ref1 = {}, ref2 = {}", d!(v, ref1), d!(w, ref2));
        *dmut!(v, ref1) = 10;
        *dmut!(w, ref2) = 11;

        println!("f(v, ref1) = {}", f(&v, ref1));

        // Should error.
        //*dmut!(w, ref1) = 12;
    }

    #[test]
    fn test_frozen_vec() {
        let v = vec![1, 2, 3, 4];
        let w = vec![5, 6, 7, 8];

        let mut v = freeze!(FrozenVec, v);
        let mut w = freeze!(FrozenVec, w);

        let ref1 = deferred!(v, 0);
        let ref2 = deferred!(w, 0);

        println!("ref1 = {}, ref2 = {}", d!(v, ref1), d!(w, ref2));
        *dmut!(v, ref1) = 10;
        *dmut!(w, ref2) = 11;

        // Should error.
        //*dmut!(w, ref1) = 12;
        // Should error.
        //v.push(100);
    }
}
