# Traits

shared behaviour (similar to interfaces)

```rust
pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{},  by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{},  by {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

fn main() {
    let tweet = Tweet {
        content: String::from("Hello world"),
        username: String::from("@johndoe"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        content: String::from("The sky is falling"),
        headline: String::from("The is not actually falling"),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
}
```

## Default Implementations

provide a method implementation in the trait block

the types reusing it will override it with their own implementation
or will use the default implementation if there is none.

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more)")
    }
}

```

### Default implementations can call other methods of the trait

e.g

```rust
pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{},  by {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    // summarize calls the summarize_author method
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

fn main() {
    let tweet = Tweet {
        content: String::from("Hello world"),
        username: String::from("@johndoe"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        content: String::from("The sky is falling"),
        headline: String::from("The is not actually falling"),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
}
```

## Traits as parameters (trait bound)

This method accepts any object that implements the `Summary` trait

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

the `&impl` syntax is syntactic sugar for something called trait bounds

that looks like this

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

trait bound syntax can be useful when we need the same type for all params

```rust

pub fn notify(item1: &impl Summary, item2: &impl Summary) {
    // ...
}

// this is more concise
pub fn notify<T: Summary>(item1: &T, item2: &T) {
    // ...
}
```

we can also make a param implement two traits

```rust
pub fn notify(item1: &(impl Summary + Display), item2: &impl Summary) {
    // ...
}

pub fn notify<T: Summary + Display>(item1: &T, item2: &T) {
    // ...
}
```

### `where` clause

specifying multiple traits can decrease readability of the function

e.g

```rust
pub fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    // ...
}
```

using the `where` clause can increase readability,
we can specify the types of the generics after the return type

```rust
pub fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
}
```

## Returning traits

a method that returns any type that implements the Summary trait

```rust
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{},  by {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

fn return_summarizable() -> impl Summary {
    Tweet {
        content: String::from("Hello world"),
        username: String::from("@johndoe"),
        reply: false,
        retweet: false,
    }
}

fn main() {
    println!("{}", return_summarizable().summarize());
}
```

## Conditionally implementing methods

here the Pair will have new method for all instances
and cmp_display method only if it implements the Display and PartialOrd traits

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {}
```
