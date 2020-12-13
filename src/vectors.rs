use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    numbers[2] = 12;

    numbers.push(6);

    numbers.pop();

    println!("{:?}", numbers);

    println!("{}", numbers[0]);

    println!("len: {}", numbers.len());

    // Arrays are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // slice
    let slice: &[i32] = &numbers[0..2];
    println!("slice: {:?}", slice);

    // loop throught vectors values
    for x in numbers.iter() {
        println!("number: {}", x);
    }

    for x in numbers.iter_mut() {
        *x *=2;
    }

    println!("numbers vec: {:?}", numbers);


}