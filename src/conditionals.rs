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
}