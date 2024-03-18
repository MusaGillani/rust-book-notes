# Vectors

```rust
    let a = [1, 2, 3];
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    // vec and its elements will be droppped outside of this scope
    {
        // macro to specify values when initializing vector
        let v2 = vec![1, 2, 3];
    }
```

## Accessing Vectors

```rust
    // Accessing vectors
    let v = vec![1, 2, 3, 4, 5];

    // unsafe, can crash at run time
    // if an index is out of bounds
    // since length of vectors is not known at compile time
    // otherwise we would get a compile time error
    let third = &v[2];
    // let third = &v[20]; will crash
    println!("The third element is {}", third);

    // To handle the run time error,
    // there is a safer way to access the vector
    // using the .get() method

    // .get() returns an Option enum
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }
```

## Iterating over vectors

```rust
    // iterating over vector;
    let v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("{}", i);
    }

```

```rust
    // iterating and mutating the original reference
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &mut v {
        // * is derefernce operator
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }
```

## Enums in Vectors

```rust
    // storing enums in vectors

    // vectors can only store one type of data

    // Scenario
    // want to represent a row of a spread sheet
    // that can store either an Int, Text or Float

    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];

    match &row[1] {
        SpreadSheetCell::Int(i) => println!("{}", i),
        _ => println!("Not an integer"),
    }
```

# Strings

Strings are stored as a collection of UTF-8 encoded bytes

## Creating String

```rust
    // creating an owned string
    let s1 = String::new();
    // string slice "&str"
    let s2 = "initial contents";
    // convert a string to slice to own string using .to_string() method
    let s3 = s2.to_string();
    // creating an owned string from a slice using the from() method
    let s4 = String::from("initial contents");
```

## Appending to strings

```rust
    // strings are just like vectors

    let mut s = String::from("foo");

    // .push_str to push a string slice "&str"
    s.push_str("bar");
    // .push() to append a character
    s.push('!');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // Appending using +

    // here we're passing ownership of s1 to s3
    // and also appending a copy of s2 by referencing its value
    // this saves a little memory compared to copying both strings
    let s3 = s1 + &s2;
    // since s1 is moved to s3, we cannot use s1 now

    println!("{}", s3);

    // Can also concatenate using the format macro
    // format macro does not take ownership, can still use s3,s2 else where
    let s4 = format!("{}{}", s3, s2);
```

## Accesing characters

```rust
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // Accessing string characters
    let namaste = String::from("नमस्ते");

    // bytes representation
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
    // 224, 165, 135]
    for b in namaste.bytes() {
        println!("{}", b);
    }

    // Scalar (char representation)
    // ['न', 'म', 'स', '्', 'त', 'े']
    for c in namaste.chars() {
        println!("{}", c);
    }

    // Grapheme Clusters (how humans perceive characters)
    // ["न", "म", "स्", "ते"]
    // no builtin way in the rust stdlib
    // need a crate for that
    for g in namaste.graphemes(true) {
        println!("{}", g);
    }
}

```

# Hashmaps

```rust
use std::collections::HashMap;

fn main() {
    // Hashmaps
    // KV pairs

    let blue = String::from("blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    // strings are moved
    // Hashmap now has ownership of these strings
    scores.insert(blue, 1);
    scores.insert(yellow, 50);

    let team_name = String::from("Blue");

    // Accessing a KV
    // .get accepts a string slice
    // .get returns an Option enum
    let score = scores.get(&team_name);

    match score {
        Some(v) => println!("Value: {}", v),
        None => println!("No values exists for key: {}", team_name),
    }

    for (key, value) in scores {
        println!("{}: {}", key, value);
    }
}

```

## Inserting Values in Hashmaps

```rust
let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    // below statement will overwrite the value
    scores.insert(String::from("Blue"), 20);

    // .entry() will return an Enum for the value of this key
    // we can call methods on this Enum
    // .or_insert() will insert a value for the key if there is no value
    scores.entry(String::from("Yellow")).or_insert(30);
    // below line will not overwrite since there already is a value for key "Yellow"
    scores.entry(String::from("Yellow")).or_insert(40);
```

## Updating values using an old value

```rust
use std::collections::HashMap;

fn main() {
    // Storing the count of each word in KV map
    let text = "Hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // .to_insert() returns a mutable reference to our value
        // &mut i32 in this case
        let count: &mut i32 = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

```
