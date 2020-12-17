pub fn run() {
    greeting("Hello", "Ciri");
    let get_sum = add(5,3);
    println!("sum: {}", get_sum);

    // closure
    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("C sum: {}", add_nums(3,3));

    let n3 = 10;
    let add_nums2 = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C sum: {}", add_nums2(3,3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name)
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn some_function(par_a: f32, par_b: i128) -> f32 {
    // 10 as f32;
    // 10f32
    // 10_f32
    10.
}