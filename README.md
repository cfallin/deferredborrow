Some meditations on deferred borrowing.

Main idea: deferred borrowing
-----------------------------

A Rust borrow is good for three reasons:

1. Safety from dangling references: the lifetime of the borrow is constrained
   to within the lifetime of the original (stack- or heap-allocated) object.

2. Safety from aliasing mutable references: a mutable borrow is disjoint from
   other borrows or direct accesses inside the lexical scope that created the
   borrow, and reborrowing preserves the invariant.

3. Safety from atomicity violations: from the moment the borrow is taken until
   its lifetime ends, it enforces its exclusivity invariants: an immutable
   borrow disallows any mutable borrow or direct mutation of the underlying
   object, while a mutable borrow disallows any other access. For this reason,
   a borrow can be implemented with just a pointer (and is thus efficient) even
   if it points to the internals of a data structure, such as a HashMap or Vec
   slot.

Sometimes benefit 3 (atomicity) is not required: we wish to hand out references
to objects stored in a vector or hashmap, but not necessarily exclusively lock
that vector or hashmap. It is enough, in principle, to hand out context to
*defer* the borrowing until later: "this Vec, slot i" or "this HashMap, key k".

Let's abstract this into a trait DeferredBorrow. This is just data; it does not
borrow the base object. At some later time, we can use the DeferredBorrow
object and the base object together to actually perform the borrow.

By itself, this gives us benefits 1 and 2 (at the time of deferred borrow, we
just use an actual borrow of the base object), but does not give us a type-safe
connection between the context and the object that handed it out. For that, we
need dependent types. (Introduce x:T notation, and desugaring on the trait.)

For further thought / for the writeup
-------------------------------------

Pointers on the heap are used in a few different ways in languages with no
aliasing restrictions (C/C++, most GC'd languages):

1. Ownership (downward) relation
2. Data structure links: up-pointers, neighbor-pointers.
3. Arbitrary graph from problem domain: DAG, directed graph.
4. Secondary indices or roots: a worklist of nodes, an index by another key.

Case studies:
- Dependency graph in a build tool
- CPU simulator
- GUI widget toolkit
