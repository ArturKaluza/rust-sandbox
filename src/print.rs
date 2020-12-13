pub fn run() {
    println!("{} {}!", "Hello", "World");

    // positional arguments
    println!("{0} {1} from {2}", "Hello", "World", "Rust");

    // named arguments
    println!(
        "My name is {name} and I like to {activity}",
        name = "Artur",
        activity = "code"
    );

    // placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (12, true, "hello"));
}
