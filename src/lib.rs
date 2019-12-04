mod vec;
pub use vec::*;

mod hashmap;
pub use hashmap::*;

pub trait DefBorrow<Base, T> {
    /// Carry out the deferred borrow, given the base object we're borrowing from.
    fn def_borrow<'a>(&self, base: &'a Base) -> &'a T;
 
    fn def_borrow_mut<'a>(&self, base: &'a mut Base) -> &'a mut T;
}

pub trait MaybeDefBorrow<Base, T> {
    fn maybe_def_borrow<'a>(&self, base: &'a Base) -> Option<&'a T>;

    fn maybe_def_borrow_mut<'a>(&self, base: &'a mut Base) -> Option<&'a mut T>;
}

impl<Base, T, D> MaybeDefBorrow<Base, T> for D
    where D: DefBorrow<Base, T> {

    fn maybe_def_borrow<'a>(&self, base: &'a Base) -> Option<&'a T> {
        Some(self.def_borrow(base))
    }

    fn maybe_def_borrow_mut<'a>(&self, base: &'a mut Base) -> Option<&'a mut T> {
        Some(self.def_borrow_mut(base))
    }
}

#[macro_export]
macro_rules! freeze {
    ($t:tt, $e:expr) => ({
        struct Tag {}
        $t::new($e, Tag {})
    });
}

#[macro_export]
macro_rules! deferred {
    ($cont:expr, $($params:expr),*) => ({
        $cont.deferred($($params),*)
    });
}

#[macro_export]
macro_rules! d {
    ($cont:expr, $e:expr) => (
        $e.def_borrow(&$cont)
    );
}

#[macro_export]
macro_rules! dmut {
    ($cont:expr, $e:expr) => (
        $e.def_borrow_mut(&mut $cont)
    )
}
