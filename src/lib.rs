pub trait DeferredBorrow<'a, Base, T> {
    /// Carry out the deferred borrow, given the base object we're borrowing from. The lifetime of
    /// the returned borrow is within (outlived by) the borrows on both `self` (the deferred borrow
    /// context) and `base` (the base object we're borrowing from). Both of those borrows, in turn,
    /// are outlived by the lifetime parameter associated with the deferred-borrow type.
    fn deferred_borrow<'this, 'base, 'ret>(&'this self, base: &'base Base) -> &'ret T
    where
        'this: 'ret,
        'base: 'ret,
        'a: 'base,
        'a: 'this;
}

// TODO: mut version.

// TODO: Option and OptionMut versions.
