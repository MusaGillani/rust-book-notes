enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // Ref , deref using pointers
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Ref, deref using smart pointers
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // ^ we set y to be an instance of a Box<T> pointing to a copied value of x rather than a reference pointing to the value of x

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // deref coercion

    let m = MyBox::new(String::from("Rust"));

    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

use std::ops::Deref;

/// Custom Smart Pointer type similar to Box<T>
/// crucial difference: this doesn't store data on heap, unlike Box<T>
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
