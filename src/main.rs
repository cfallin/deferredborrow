mod lib;
use lib::*;

// Macros for:
// - deferred borrow objects of struct fields
//
// Impls for:
// - Vec, HashMap, ... (returning Option<&T> instead? WeakDeferredBorrow? A strong variant that
// comes from a wrapper around the Vec/HashMap that prevents structural mutation that could
// invalidate things? NoDropVec, NoDropHashMap. Another wrapper type for Box?)
//
// Compiler plugin for:
// - Generating a thunk around a closure.
//
// Type-system extension for:
// - dependent borrows: for a value y of type x:T, support desugaring y.deref() to
// y.deferred_borrow(&x).
//   - dynamic checks (unique ID in obj header and in deferred borrow ctxs?) if not.
//     perhaps only enable in debug mode.
//
// Examples using self.cpus:VecRef (jsim) or self.nodes:NodeRef. In latter case, cite conventional
// wisdom of using node IDs rather than pointers; note that this is still type-unsafe (index can be
// used on any vec; vec might be shrunk or reordered). NonShrinkingVec as a wrapper that can give
// out DeferredBorrows. Same for HashMap, but needs to re-lookup hash. FrozenKeyHashMap can give
// out actual ptrs inside the contexts.

struct S {
    pub x: u32,
    pub y: u32,
}

struct DeferredBorrowX {}
struct DeferredBorrowY {}

impl<'a> DeferredBorrow<'a, S, u32> for DeferredBorrowX {
    fn deferred_borrow<'this, 'base, 'ret>(&'this self, s: &'base S) -> &'ret u32
    where
        'a: 'this,
        'a: 'base,
        'this: 'ret,
        'base: 'ret,
    {
        &s.x
    }
}

fn main() {
    let mut s = S { x: 1, y: 2 };
    let b = DeferredBorrowX {};

    s.x = 42;
    println!("{}", b.deferred_borrow(&s));
    s.x = 84;
    println!("{}", b.deferred_borrow(&s));
}
