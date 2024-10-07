enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("counter after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a)); // not a deep clone, increments reference count for a
    println!("counter after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("counter after creating c = {}", Rc::strong_count(&a));

        // Drop trait decreases the reference count automatically
        // when a Rc<T> value goes out of scope
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
