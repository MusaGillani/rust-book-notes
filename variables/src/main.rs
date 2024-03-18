fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    let tup = (1, 'm', "musa", true);
    let unit_tup = ();

    // shadowing
    let y = 5;

    let y = "musa"; // still mutable but shadowed, now y is always a string

    // destructuting a tuple
    let (number, char, name, _condition) = tup;

    my_function();
}

fn my_function() {
    println!("another function");
}
