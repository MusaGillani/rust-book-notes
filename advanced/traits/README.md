# Associated Types

connect a type placeholder with the trait,
such that the trait method defs can use the placeholder type in their signatures

```rust

pub trait Iterator {
    type: Item;

    fn next(&mut self) -> Option<Self::Item>;
}

```

## Difference between Generics and Associated Types

an trait with generic can implemented multiple times for a type

with associated types, the type is only implemeneted once.
