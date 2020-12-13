pub fn run() {
    let x = 1;
    let y: i32 = 2;

    println!("max i32: {}", std::i32::MAX);
    println!("max i64: {}", std::i64::MAX);

    let is_active = true;

    let a1 = 'a';
    // unicde is one char 
    let face = '\u{1F600}';

    println!("{:?}", (x, y, is_active, a1, face));
}