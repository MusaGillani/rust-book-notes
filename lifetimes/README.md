# Lifetimes

> TL;DR: lifetimes are how long a reference lives annotated in syntax

Dangling references :
reference that points to invalid data

Lifetime shown:

```rust
fn main() {
    let r;                // ---------+-- 'a
                        //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                        //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

Error:

```rust
fn main() {
    let r;

    {
        let x = 5;
        r = &x; // error !
    }

    println!("r: {}", r); //here r is dangling reference
    // because x is destoyed when it is out of scope and r keeps a reference to that value
}
```

luckily the borrowchecker checks this value and does not let us compile.
because it knows x will be destroyed.

## What if the borrow checker cannot determine the lifetime?

```rust
fn longest(x: &str, y: &str) -> & str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

In the above code, `x` and `y` can come from many different places and be passed to this function
and we cannot know which one of them will be returned.

The compiler will throw an error saying:

> missing lifetime specifier
> this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`

In scenarios where the borrow checker cannot determine the lifetime,
we need to specify relations of lifetimes.

## Lifetime Annotation syntax

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

Here we specify the borrow using the _generic lifetime annotation_ `'a`
the relation we specify is that the returned result will have either
lifetime same as x or y.

> Note: lifetime annotations do not change the lifetime, they just specify a relation

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

this helps the borrow checker because it will only check if the the result has a
lifetime equal to the lifetime of the smallest of x and y.

> Note: smallest of the two lifetimes because
> both lifetimes are valid for that time
> e.g x has lifetime of 10 and y has lifetime of 5
> both are valid for 5 so the smallest lifetime is picked

**now the borrow checker only has to check _the scope_ in which
result(the return value) being used has the lifetime of the smallest param available
to avoid a dangling reference.**

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    // borrow checker only needs to check the smallest of string1 and string2 are available in scope when result is used
    // here the lifetimes are valid
    println!("The longest string is {}", result); // result being used here
}
```

another example

here the smallest lifetime is of string2 which is valid when result is used

```rust
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
```

but here

string2 gets destroyed as it is out of scope,
so the smallest lifetime is not valid where result is being used

we get an error on this line:
`result = longest(string1.as_str(), string2.as_str());`

> Error: `string2` does not live long enough
> borrowed value does not live long enough

```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result); // result being used
}

```

### references returned have to match lifetime of one of the params

e.g returning a reference to a value owned/created by a function

```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

this is because the local scope variables in the function get destroyed when the all is finished

to fix this, we can returned **Owned** types i.e we transfer the ownership after returning the reference

```rust
    fn longest(x: &str, y: &str) -> String {
    let result = String::from("really long string");
    result
}
```

## Structs having Lifetime annotations

this code states the struct cannot outlive the lifetime of the reference passed to the `part` field

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
fn main() {
    let novel = String::from("Call me Ishmael. Some years ago");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

```

if we use the `i` variable after the `first_sentence` variable has gone out of scope, we will get an error

## Lifetime Elission

the compiler can sometimes determine the lifetimes of the references.

e.g

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

this functions compiles even without the lifetime annotations.

> input lifetimes: lifetimes of arguments
> output lifetimes: lifetimes of return values

the compiler does so using three rules:

1. Each parameter that is a reference gets its own lifetime parameter

2. If there is exactly one input lifetime parameter, that lifetime is
   assigned to all output lifetime parameters;

3. If there are multiple input lifetime parameters, but one of them is
   &self or &mut self the lifetime of self is assigned to all output
   lifetime parameters.

so in our example, using rule two we know the liftime of the returned reference

## Lifetime Annotations in `Impl` blocks

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

```

similarly: lifetime annotations are not _required_ due to rule 3 being applicable here
but we still can specify a lifetime

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // note: don't need to specify generic in the function param
    // its already in scope from the struct generic
    fn return_part(&'a self, announcement: &str) -> &'a str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

```

## `static` lifetime

the reference can live as long as the duration of the program

e.g

all string literals have a static lifetime (string literals are stored in the program binary)

```rust
let s: &'static str = "I have a static lifetime.";
```

# Generic Type Parameters, Trait bounds, and Lifetimes together

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where // trait bound restricting the generic 'T' to only be those implementing `Display`
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
