use std::collections::HashMap;

fn main() {
    // Storing the count of each word in KV map
    let text = "Hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // .to_insert() returns a mutable reference to our value
        // &mut i32 in this case
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
