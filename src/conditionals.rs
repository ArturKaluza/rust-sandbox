pub fn run() {
    let age: u8 = 18;
    let check_id = true;

    if age >= 21 && check_id {
        println!("Bartender: What do you like to drink");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave");
    } else {    
        println!("Bartender: I'll need to see your ID");
    }

    let is_of_age = if age >= 21 { true } else { false };
    println!("{}", is_of_age);


    let some_int = 7;
    let some_int2 = -3;

    let var_from_inline = if some_int == 9 {
        300
    } else if some_int2 == -3 {
        0
    } else {
        400
    };

    let some_bool = true;

    match some_bool {
        true => {
            println!("hit true branch");
        }
        false => {
            println!("hit false branch");
        }
    };

    let some_int = 10;

    match some_int {
        0 => println!("0"),
        1 | 2 => println!("1 or 2"),
        3..=100 => println!("3-100"),
        _ => println!("else branch")
    };
}