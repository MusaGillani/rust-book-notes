# Generics

## using with functions

```rust
fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut larget = number_list[0];
    for number in number_list {
        if number > larget {
            larget = number;
        }
    }
    larget
}
```

## using with structs

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let p3 = Point { x: 5, y: 10.0 };
}
```

## using with enums

the two most used enums `Result` and `Option` both are generic enums.

```rust
fn main() {
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        OK(T),
        Err(E),
    }
}
```

## Generics with implementation blocks

```rust
struct Point<T> {
    x: T,
    y: T,
}

// The generic type can be anything else than T
// i.e doesn't have to be the same as that of Point
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// This implementation block is for concrete types only
// i.e only when x and y both are floats that this method is available
impl Point<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.1, y: 2.2 };
}
```

### An example swapping or mixing values

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

// This implementation block is for the Point struct
// the method it defines is mixup
// mixup takes different generics than that of Point
// this indicates mixup takes in a Point different than the instance its been called on
// Mixup defines a return type of a Point that has x from the instance its called on and y from the instance it is passed
// This can be seen from the Return statement
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    // output
    // p3.x = 5, p3.y = c
}
```

## Performance

using generics reduces the number of definitions required

e.g

```rust
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let integer  = Option::Some(5);
    let float = Option::Some(5.0);
}
```

this saves us defining two option enums

but at compile time rust will make 2 option enums for us based on the usage in main

the code will become

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

so there is no performance overhead
