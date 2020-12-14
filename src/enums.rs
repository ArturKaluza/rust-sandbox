enum Movment {
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movment) {
    match m {
        Movment::Up => println!("Avatar moving up"),
        Movment::Down => println!("Avatar moving down"),
        Movment::Left => println!("Avatar moving left"),
        Movment::Right => println!("Avatar moving right"),
    }
}

pub fn run() {
    let avatar1 = Movment::Left;
    let avatar2 = Movment::Up;
    let avatar3 = Movment::Right;
    let avatar4 = Movment::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}
