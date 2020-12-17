pub fn run() {
    // immutable(exepxcions) fixed length
    // stored in heap
    let hello_exe = "Hello";
    let world = "World";

    // mutable
    let mut hello = String::from("Hello");

    // &str -> String
    let s = hello_exe.to_string();
    let _sexe2 = "String exapmple".to_string();
    let _sexe3 = String::from(world);

    let _str_from_string: &str = &world;

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

    // let test = "first" + "second";
    let combine_string_literals = ["first", "second"].concat();
    let format_marco = format!("{} {}", "first", "second");

    // string must be first;
    let string_plus_str = s + world;

    let mut mut_string = String::new();
    mut_string.push_str("Some hardcoded literal");
    mut_string.push('m');

    let str_from_substring: &str = &world[0..2];
    let str_from_substring2: &str = &world[0..=2];

    let char_by_index = &world.chars().nth(1);

    match char_by_index {
        Some(c) => println!("Found a char {}", c),
        None => {}
    }

    if let Some(c) = world.chars().nth(2) {
        println!("Found a char {}", c);
    }
;}