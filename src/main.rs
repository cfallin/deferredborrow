mod lib;
use lib::*;

mod vec;
use vec::*;

fn f<Tag>(v: &AppendOnlyVec<usize, Tag>, ref1: AppendOnlyVecRef<usize, Tag>) -> usize {
    *d!(v, ref1)
}

fn main() {
    let v = vec![1,2,3,4];
    let w = vec![5,6,7,8];

    let mut v = freeze!(AppendOnlyVec, v);
    let mut w = freeze!(AppendOnlyVec, w);

    let ref1 = deferred!(v, 0);
    let ref2 = deferred!(w, 0);

    println!("ref1 = {}, ref2 = {}", d!(v, ref1), d!(w, ref2));
    *dmut!(v, ref1) = 10;
    *dmut!(w, ref2) = 11;

    println!("f(v, ref1) = {}", f(&v, ref1));

    // Should error.
    //*dmut!(w, ref1) = 12;
}
