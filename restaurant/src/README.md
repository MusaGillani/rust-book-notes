```rust
mod front_of_house {
    // need to make hosting public
    // by default it is a private module
    // private causes front_of_house to not see anything in its children
    // by hosting can still see anything defined in its parent
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    // crate is the parent module of our project created that nests all other modules
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // calling a function from its parent module, which in this case is `crate`
        super::serve_order();
    }

    fn cook_order() {}
}

```

# using structs

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("burnt");

    meal.toast = String::from("wheat");
}
```

# use keyword

```rust

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// use brings the module path in scope
use crate::front_of_house::hosting;
// relative path
use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // without the use keyword
    front_of_house::hosting::add_to_waitlist();
    // with use
    hosting::add_to_waitlist();
}
```

# Modules in different files

To export usage of our modules to crates external than self

`lib.rs`

```rust
pub use crate::front_of_house::hosting;
```

to move modules to different files for example

`lib.rs`

```rust
mod front_of_house;

pub use crate::front_of_house::hosting;
```

create a file called front_of_house.rs in the same dir as lib.rs

`front_of_house.rs`

```rust

pub mod hosting {

}
```

to further break the hosting module to another file

create a _dir_ called `front_of_house` next to the `front_of_house.rs` file
and create a `hosting.rs` file

`front_of_house/hosting.rs`

```rust
pub fn xyz(){

}
```

> TL;DR

1. create file names same as module for single file
2. create folder names same as modules for mutliple files
