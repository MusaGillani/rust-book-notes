# Error handling in rust

## Panic Macro

program fails in a way that is unrecoverable
or an error that cannot be handled gracefully

```rust
    // Immediately quit the program
    // and print this error message
    panic!("Crash and Burn");
```

## Result Enum

just like the Option enum,
this shows if there is an error or not

```rust
    enum Result<T,E {
        Ok(T),
        Err(E),
    }
```

example usage

```rust
use std::fs::File;
fn main() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problems opening the file: {:?}", error),
    };
}

```

## Handling Specific Erros using the ErrorKind enum

```rust
use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("Hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            // if there is any other error,
            // bind or store it in this variable
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

```

instead of writing this

```rust
 use std::fs::File;
fn main() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(e) => panic!("Problem creating the file: {:?}", e),
    };
}
```

we can simplify it using .unwrap()

```rust
    let f = File::open("hello.txt").unwrap();
```

this is the same as calling the above match statement as
`unwrap()` will return the value passed to Result::Ok()
or panic if there is an Err()

Similary calling `.expect(msg)` works the same way as unwrap() but
panics with the error message provided

## Error Propagation

passing around errors rather than handling them

our function will return the Err instead of handling it

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

### Using the ? operator

Above can be further simplified using the `?` operator

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

both these method are identical, the `?` operator will _return_ the error from the function if there is one, otherwise the function will continue as normal, hence why we're returngin the Ok(s) enum

to even further simplify this function,
we can use a method directly from the fs module:

```rust
use std::fs::{self};
use std::io::{self};

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

this function does basically the same thing as the 3 lines above
and returns a result enum with the file contents as a string, or an err if there is one
