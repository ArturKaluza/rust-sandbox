pub fn run() {
   let stack_f64: f64 = 1.;
   let heap_f64: Box<f64> = Box::new(2.);

    stack_procedure(stack_f64);
    println!("In main stack {}", stack_f64);

    heap_procedure(&heap_f64);
    println!("In main heap {}", heap_f64);

    let some_string = String::from("Artur");
    let some_str: &str = "Partner"; 

    some_proc(&some_string, some_str);
    println!("{} {}", some_string, some_str);
}

fn stack_procedure(mut param: f64) {
    param += 9.;
    println!("In stack with param {}", param);
}

fn heap_procedure(param: &Box<f64>) {
    println!("In heap with param {}", param);
}

fn some_proc(a: &String, b: &str) {
    println!("{} {}", a, b);
}