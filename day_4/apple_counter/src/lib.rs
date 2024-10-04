//! In this exercise, you will practice the concepts of drop, shared ownership
//! and interior mutability.
//!
//! Let's start with a fun story to motivate ourselves:
//!
//! Jacob is an apple farmer who likes to maximize efficiency.
//! When he loads his truck with apples he stacks them real high!
//! Unfortuntely, the truck is so full that usually some apples fall off.
//! Jacob would like to know how many of them do.
//!
//! Here are some links where you can read up on the relevant topics:
//! - shared ownership:
//!   - book chapter: https://doc.rust-lang.org/stable/book/ch15-04-rc.html
//!   - std lib docs: https://doc.rust-lang.org/stable/std/rc/struct.Rc.html
//! - interior mutability:
//!   - book chapter: https://doc.rust-lang.org/stable/book/ch15-05-interior-mutability.html
//!   - std lib docs: https://doc.rust-lang.org/stable/std/cell/struct.RefCell.html
//! - Drop trait:
//!   - book chapter: https://doc.rust-lang.org/stable/book/ch15-03-drop.html
//!   - std lib docs: https://doc.rust-lang.org/stable/std/ops/trait.Drop.html

// The exercise forces you to use smart pointers, so it needs to hide
// implementation details of the "truck" from you, so you can't just read how
// many apples are left in the truck. That's the purpose of this trait.
//
// The type of the apple is a generic parameter, you are flexible here.
// Can you create a type that allows you to detect when it is dropped?
//
pub trait Truck<Apple> {
    fn load(&mut self, apple: Apple);

    // During delivery, the truck will drop some apples.
    fn deliver(&mut self);
}

// TODO: specify the type of your apple here!
//                                   vv
pub fn count_dropped_apples<T: Truck<()>>(truck: &mut T) -> usize {
    for _ in 0..10_000 {
        //
        // TODO: load the truck with your custom apples!
        //          vv
        let apple = ();
        truck.load(apple);
    }
    // This is where the truck will drop some apples:
    truck.deliver();

    todo!("count the number of apples dropped during delivery")
}
