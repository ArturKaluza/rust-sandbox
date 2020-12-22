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

enum Payment {
    Cash(f32),
    CreditCard(String, f32),
    DebitCard(DebitData),
    Crypto{account_id: String, amount: f32},
}

struct DebitData {
    pub card_number: String,
    pub amount: f32,
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

    let some_payment = Payment::Cash(100.);
    process_payment(some_payment);

    let cc_payment = Payment::CreditCard("CC Num".to_string(), 250.);
    process_payment(cc_payment);
   
    let debit_payment = Payment::DebitCard(DebitData {
        card_number: "Debit num".to_string(),
        amount: 400.,
    });
    process_payment(debit_payment);

    let crypto_payment = Payment::Crypto{account_id: "test abt".to_string(), amount: 1000.};
    process_payment(crypto_payment);
}

fn process_payment(some_payment: Payment) {
    match some_payment {
        Payment::Cash(amt) => {
            println!("Paying with cash...{}", amt);
        }
        Payment::CreditCard(some_string, _some_f32) => {
            println!("Paying with card...{}", some_string);
        }
        Payment::DebitCard(data) => {
            println!("Debit card {} {}", data.amount, data.card_number);
        }
        Payment::Crypto{account_id, amount} => {
            println!("Paying with crypto {} {}", account_id, amount);
        }
        // _ => {

        // }
    }
}
