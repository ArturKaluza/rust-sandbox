pub fn run() {
    let person: (&str, &str, i8) = ("Artur", "Poland", 33);

    println!("{} is from {} and is {}", person.0, person.1, person.2);

    let some_tuple = (2, 3.4, (1.1, 2.2));
    println!("My data is {} {}", some_tuple.0, some_tuple.1);
    println!("My full tuple is {:?}", some_tuple);

    let some_val = (some_tuple.2).1;

    // populaqting multiple variable from tuples
    // let some_color = get_some_rgb();
    // let (my_red, my_green, my_blue) = some_color;

    let (my_red, my_green, my_blue) = get_some_rgb();
    println!("r {} g {} b {}", my_red, my_green, my_blue);


    let some_other_color: (u8, u8, u8, u8) = (0, 100, 150, 255);

    let empty_tuple = ();
}

fn get_some_rgb () -> (u8, u8, u8) {
    (200, 100, 20)
}

