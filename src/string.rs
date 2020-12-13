pub fn run() {
    // immutable fixed length
    let _hello_exe = "Hello";

    let mut hello = String::from("Hello");

    println!("length: {}", hello.len());

    hello.push(' ');
    hello.push('W');

    hello.push_str("orld");

    // capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // isEmpty
    println!("Is empty: {}", hello.is_empty());

    // contains
    println!("Contains 'world' {}", hello.contains("World"));

    println!("Replace: {}", hello.replace("World", "There"));

    // loop through string by white space
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // assertion testing
    assert_eq!(2, s.len());

    println!("{}", hello);

}