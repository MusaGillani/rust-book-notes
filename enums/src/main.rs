fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // match expressions are exhaustive, i.e we have to match all arms.
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn if_let() {
    let some_value = Some(3);
    match some_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // instead of the above syntax
    // this is not exhaustive.
    if let Some(3) = some_value {
        println!("three")
    }
}
