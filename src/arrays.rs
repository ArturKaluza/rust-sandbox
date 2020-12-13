use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    numbers[2] = 12;

    println!("{:?}", numbers);

    println!("{}", numbers[0]);

    println!("len: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // slice
    let slice: &[i32] = &numbers[0..2];
    println!("slice: {:?}", slice);


}