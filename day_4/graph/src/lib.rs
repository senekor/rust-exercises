//! In this exercise, you will implement a graph data structure in Rust.
//! This is a notoriously hard problem, because graphs can contain cycles,
//! meaning it's not clear who owns which node. Of course, it's possible
//! to represent graphs in a way that circumvents this problem, but we'll
//! do it the hard way.
//!
//! This graph is going to be UNDIRECTED, meaning there is no difference
//! between "A is a neighbor of B" and "B is a neighbor of A".
//!
//! I considered simply storing `i32` or something in each node. But we
//! have learned about generics, so let's use that! A graph should be
//! able to store arbitrary types. However, there are some assumptions
//! we need to make about the elements:
//!
//! - Elements must be clonable, otherwise we have to deal with references.
//!   That would be extremely difficult in combination with `RefCell`.
//!
//! - Elements must be comparable, so the tests can check if one graph
//!   is the same as another.
//!
//! - You CANNOT assume elements are hashable or orderable.

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Clone)]
struct RawNode<T: Clone + Eq> {
    /// This id is never changed, it only serves the purpose of making sure two
    /// different nodes with the same value are not considered equal.
    value: T,
    neighbors: Vec<WeakNode<T>>,
    tracker: Rc<RefCell<Tracker<T>>>,
}
impl<T: Clone + Eq> PartialEq for RawNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}
impl<T: Clone + Eq> Eq for RawNode<T> {}

impl<T: Clone + Eq> RawNode<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            neighbors: Vec::new(),
            tracker: Rc::new(RefCell::new(Tracker::default())),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Tracker<T: Clone + Eq> {
    nodes: Vec<Node<T>>,
}
impl<T: Clone + Eq> Default for Tracker<T> {
    fn default() -> Self {
        Self { nodes: Vec::new() }
    }
}

// We wrap the smart pointers for shared ownership and interior mutability
// in this type to hide them as implementation details.
#[derive(Clone, PartialEq, Eq)]
pub struct Node<T: Clone + Eq>(Rc<RefCell<RawNode<T>>>);

impl<T: Clone + Eq> Node<T> {
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(RawNode::new(value))))
    }

    /// returns the value stored in the node
    pub fn value(&self) -> T {
        RefCell::borrow(&self.0).value.clone()
    }

    /// returns an iterator over all neighboring nodes
    pub fn neighbors(&self) -> impl Iterator<Item = Self> {
        // // TODO (can't use todo macro with this return type)
        // [].into_iter()
        let v = RefCell::borrow(&self.0)
            .neighbors
            .iter()
            .map(WeakNode::upgrade)
            .collect::<Vec<_>>();
        v.into_iter()
    }

    /// Our graph should be able to mutate internally.
    /// This method provides that capability.
    pub fn update_value(&self, f: fn(T) -> T) {
        let value = self.value();
        RefCell::borrow_mut(&self.0).value = f(value);
    }

    pub fn add_neighbor(&self, neighbor: Self) {
        let contains = |v: &[WeakNode<T>], neighbor| v.iter().any(|n| &n.upgrade() == neighbor);
        {
            let v = &mut RefCell::borrow_mut(&self.0).neighbors;
            if !contains(v, &neighbor) {
                v.push(neighbor.downgrade());
            }
        }
        let v = &mut RefCell::borrow_mut(&neighbor.0).neighbors;
        if !contains(v, self) {
            v.push(self.downgrade());
        }
    }

    pub fn remove_neighbor(&self, neighbor: Self) {
        {
            let v = &mut RefCell::borrow_mut(&self.0).neighbors;
            v.retain(|elem| elem.upgrade() != neighbor);
        }
        let v = &mut RefCell::borrow_mut(&neighbor.0).neighbors;
        v.retain(|elem| elem.upgrade() != *self);
    }

    fn downgrade(&self) -> WeakNode<T> {
        WeakNode(Rc::downgrade(&self.0))
    }
}

#[derive(Clone)]
pub struct WeakNode<T: Clone + Eq>(Weak<RefCell<RawNode<T>>>);

impl<T: Clone + Eq> WeakNode<T> {
    fn upgrade(&self) -> Node<T> {
        Node(Weak::upgrade(&self.0).expect("reachable neighbors should not have been dropped"))
    }
}
