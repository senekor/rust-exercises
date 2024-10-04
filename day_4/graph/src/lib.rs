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
//! - You CANNOT assume elements are hashable or comparable.

use std::{cell::RefCell, rc::Rc};

#[derive(Clone, PartialEq, Eq, PartialOrd)]
struct RawNode<T> {
    /// This id is never changed, it only serves the purpose of making sure two
    /// different nodes with the same value are not considered equal.
    value: T,
    neighbors: Vec<Node<T>>,
}

impl<T> RawNode<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            neighbors: Vec::new(),
        }
    }
}

// We wrap the smart pointers for shared ownership and interior mutability
// in this type to hide them as implementation details.
#[derive(Clone, PartialEq, Eq, PartialOrd)]
pub struct Node<T>(Rc<RefCell<RawNode<T>>>);

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
        let v = RefCell::borrow(&self.0).neighbors.to_vec();
        v.into_iter()
    }

    /// Our graph should be able to mutate internally.
    /// This method provides that capability.
    pub fn update_value(&self, f: fn(T) -> T) {
        let value = self.value();
        RefCell::borrow_mut(&self.0).value = f(value);
    }

    pub fn add_neighbor(&self, neighbor: Self) {
        {
            let v = &mut RefCell::borrow_mut(&self.0).neighbors;
            if !v.contains(&neighbor) {
                v.push(neighbor.clone());
            }
        }
        let v = &mut RefCell::borrow_mut(&neighbor.0).neighbors;
        if !v.contains(self) {
            v.push(self.clone());
        }
    }

    pub fn remove_neighbor(&self, neighbor: Self) {
        {
            let v = &mut RefCell::borrow_mut(&self.0).neighbors;
            v.retain(|elem| elem != &neighbor);
        }
        let v = &mut RefCell::borrow_mut(&neighbor.0).neighbors;
        v.retain(|elem| elem != self);
    }
}
