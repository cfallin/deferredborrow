use crate::DefBorrow;

use std::borrow::Borrow;
use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use std::mem;

pub struct AppendOnlyHashMap<K, V, Tag> {
    h: HashMap<K, V>,
    _tag: PhantomData<Tag>,
}

pub struct AppendOnlyHashMapRef<K, V, Tag> {
    k: K,
    _phantom1: PhantomData<V>,
    _phantom2: PhantomData<Tag>,
}

impl<K, V, Tag> PartialEq for AppendOnlyHashMap<K, V, Tag>
where
    HashMap<K, V>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.h.eq(&other.h)
    }
}
impl<K, V, Tag> Eq for AppendOnlyHashMap<K, V, Tag> where HashMap<K, V>: Eq {}

impl<K, V, Tag> Clone for AppendOnlyHashMapRef<K, V, Tag>
where
    K: Clone,
{
    fn clone(&self) -> Self {
        AppendOnlyHashMapRef {
            k: self.k.clone(),
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }
}

impl<K, V, Tag> AppendOnlyHashMap<K, V, Tag> {
    pub fn new(h: HashMap<K, V>, _tag: Tag) -> AppendOnlyHashMap<K, V, Tag> {
        AppendOnlyHashMap {
            h,
            _tag: PhantomData,
        }
    }

    pub fn get<'a, Q: ?Sized>(&'a self, k: &Q) -> Option<&'a V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
        K: Hash + Eq,
    {
        self.h.get(k)
    }

    pub fn get_mut<'a, Q: ?Sized>(&'a mut self, k: &Q) -> Option<&'a mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
        K: Hash + Eq,
    {
        self.h.get_mut(k)
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V>
    where
        K: Hash + Eq,
    {
        self.h.insert(k, v)
    }

    pub fn get_or_insert<'a, F>(&'a mut self, k: K, f: F) -> &'a mut V
    where
        K: Hash + Eq,
        F: FnOnce() -> V,
    {
        self.h.entry(k).or_insert_with(f)
    }

    pub fn deferred(&self, k: K) -> Option<AppendOnlyHashMapRef<K, V, Tag>>
    where
        K: Hash + Eq,
    {
        if self.h.contains_key(&k) {
            Some(AppendOnlyHashMapRef {
                k: k,
                _phantom1: PhantomData,
                _phantom2: PhantomData,
            })
        } else {
            None
        }
    }

    pub fn to_hashmap(self) -> HashMap<K, V> {
        self.h
    }
}

impl<K, V, Tag> DefBorrow<AppendOnlyHashMap<K, V, Tag>, V> for AppendOnlyHashMapRef<K, V, Tag>
where
    K: Hash + Eq,
{
    fn def_borrow<'a>(&self, base: &'a AppendOnlyHashMap<K, V, Tag>) -> &'a V {
        base.h.get(&self.k).unwrap()
    }

    fn def_borrow_mut<'a>(&self, base: &'a mut AppendOnlyHashMap<K, V, Tag>) -> &'a mut V {
        base.h.get_mut(&self.k).unwrap()
    }
}

pub struct FrozenHashMap<K, V, Tag> {
    h: HashMap<K, V>,
    _tag: PhantomData<Tag>,
}

#[derive(Copy)]
pub struct FrozenHashMapRef<K, V, Tag> {
    ptr: *mut V,
    _phantom1: PhantomData<K>,
    _phantom2: PhantomData<Tag>,
}

impl<K, V, Tag> PartialEq for FrozenHashMap<K, V, Tag>
where
    HashMap<K, V>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.h.eq(&other.h)
    }
}
impl<K, V, Tag> Eq for FrozenHashMap<K, V, Tag> where HashMap<K, V>: Eq {}

impl<K, V, Tag> Clone for FrozenHashMapRef<K, V, Tag> {
    fn clone(&self) -> Self {
        FrozenHashMapRef {
            ptr: self.ptr,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }
}

impl<K, V, Tag> FrozenHashMap<K, V, Tag> {
    pub fn new(h: HashMap<K, V>, _tag: Tag) -> FrozenHashMap<K, V, Tag> {
        FrozenHashMap {
            h,
            _tag: PhantomData,
        }
    }

    pub fn get<'a, Q: ?Sized>(&'a self, k: &Q) -> Option<&'a V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
        K: Hash + Eq,
    {
        self.h.get(k)
    }

    pub fn get_mut<'a, Q: ?Sized>(&'a mut self, k: &Q) -> Option<&'a mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
        K: Hash + Eq,
    {
        self.h.get_mut(k)
    }

    pub fn deferred(&self, k: K) -> Option<FrozenHashMapRef<K, V, Tag>>
    where
        K: Eq + Hash,
    {
        if let Some(v) = self.h.get(&k) {
            Some(FrozenHashMapRef {
                ptr: unsafe { mem::transmute(v) },
                _phantom1: PhantomData,
                _phantom2: PhantomData,
            })
        } else {
            None
        }
    }

    pub fn to_hashmap(self) -> HashMap<K, V> {
        self.h
    }
}

impl<K, V, Tag> DefBorrow<FrozenHashMap<K, V, Tag>, V> for FrozenHashMapRef<K, V, Tag> {
    fn def_borrow<'a>(&self, _base: &'a FrozenHashMap<K, V, Tag>) -> &'a V {
        unsafe { mem::transmute(self.ptr) }
    }

    fn def_borrow_mut<'a>(&self, _base: &'a mut FrozenHashMap<K, V, Tag>) -> &'a mut V {
        unsafe { mem::transmute(self.ptr) }
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use std::collections::HashMap;

    fn f<Tag>(
        v: &AppendOnlyHashMap<u32, u32, Tag>,
        ref1: AppendOnlyHashMapRef<u32, u32, Tag>,
    ) -> u32 {
        *d!(v, ref1)
    }

    #[test]
    fn test_append_only_hashmap() {
        let h1: HashMap<u32, u32> = [(10, 0), (11, 1), (12, 2)].iter().cloned().collect();
        let h2: HashMap<u32, u32> = [(10, 3), (11, 4), (12, 5)].iter().cloned().collect();

        let mut h1 = freeze!(AppendOnlyHashMap, h1);
        let h2 = freeze!(AppendOnlyHashMap, h2);

        let ref1 = deferred!(h1, 10).unwrap();
        assert!(deferred!(h1, 100).is_none());
        let ref2 = deferred!(h2, 10).unwrap();

        for i in 100..110 {
            assert!(h1.insert(i, 10 * i).is_none());
        }
        assert!(h1.insert(10, 4) == Some(0));

        assert!(*d!(h1, ref1) == 4);
        assert!(*d!(h2, ref2) == 3);
        *dmut!(h1, ref1) = 5;
        assert!(*d!(h1, ref1) == 5);
        assert!(f(&h1, ref1.clone()) == 5);
    }

    #[test]
    fn test_frozen_hashmap() {
        let h1: HashMap<u32, u32> = [(10, 0), (11, 1), (12, 2)].iter().cloned().collect();
        let h2: HashMap<u32, u32> = [(10, 3), (11, 4), (12, 5)].iter().cloned().collect();

        let mut h1 = freeze!(FrozenHashMap, h1);
        let h2 = freeze!(FrozenHashMap, h2);

        let ref1 = deferred!(h1, 10).unwrap();
        assert!(deferred!(h1, 100).is_none());
        let ref2 = deferred!(h2, 10).unwrap();

        assert!(*d!(h1, ref1) == 0);
        assert!(*d!(h2, ref2) == 3);
        *dmut!(h1, ref1) = 5;
        assert!(*d!(h1, ref1) == 5);
    }
}
