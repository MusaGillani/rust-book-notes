### Deref coercion

automatically substituting types within Structs with Deref trait implemented using deref() where used
with references.

i.e

`&T` to `&U` when `T: Deref<Target U>`

that if you have a `&T`, and `T` implements Deref to some type `U`, you can get a `&U` transparently.

```rust
fn main() {
    // deref coercion
    let m = MyBox::new(String::from("Rust"));

    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

use std::ops::Deref;

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

```

without deref coercion:

```rust
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
```

The `(*m)` dereferences the `MyBox`<String> into a `String`.
Then the `&` and `[..]` take a string slice of the `String` that is equal to the whole string to match the signature of `hello`
