pub fn run() {
    let name = "Artur";
    let mut age = 33;

    age = 34;

    println!("my name is {} and I'm {}", name, age);

    // define const
    const ID: i32 = 007;
    println!("ID: {}", ID);

    //assign multiple vars
    let (name, age) = ("Artur", 33);
    println!("my name is {} and I'm {}", name, age);
}