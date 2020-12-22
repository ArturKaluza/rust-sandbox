struct Point<T> {
    x: T,
    y: T,
}

enum SomeEnum<T> {
    OptionA(T),
    OptionB(T),
    OptionC
}

trait SomeTrait {
    fn example_func(&self, a: &str, b: &str) -> String;
}

fn do_this<T>(some_var: &T) -> String
where T: SomeTrait {
    some_var.example_func("first", "second")
}

struct MyStruct {
    something: i32,
}

impl SomeTrait for MyStruct {
    fn example_func(&self, a: &str, b: &str) -> String {
        self.something.to_string() + " - " + a + " - " + b
    }
}

impl SomeTrait for i32 {
    fn example_func(&self, a: &str, b: &str) -> String {
        "i32".to_string() + " - " + a + " - " + b
    }
}

pub fn run() {
    let a = Point { x: 100, y: -1 };
    println!("x = {} y = {}", a.x, a.y);

    let b = Point { x: 10.1, y: -2.3 };
    println!("x = {} y = {}", b.x, b.y);

    let some_data = SomeEnum::OptionA(34.2);

    match some_data {
        SomeEnum::OptionA(a) => println!("OptionA {}", a),
        SomeEnum::OptionB(b) => println!("OptionB {}", b),
        SomeEnum::OptionC => println!("just c")
    }

    let some_data2 = SomeEnum::OptionB('b');
    let some_data3 = SomeEnum::OptionA(vec![1,2,3,4]);

    let d = my_func(4, 5);

    let test = MyStruct { something: 1000 };
    let result = do_this(&test);

    let testi32 = 18;
    let result2 = do_this(&testi32);

    let test5 = GenStruct {
        gen_t: 5.6,
        gen_u: vec![1,2,3]
    };

    test5.log_something();
}

fn my_func<T: std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::fmt::Debug>(input_a: T, input_b: T) -> T {
    println!("input_a has {:?}", input_a);
    input_a + input_b
}

fn my_func2<T, U>(input_a: T, input_b: T, input_e: U) -> T 
where T: std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::fmt::Debug,
      U: std::fmt::Debug   {
    println!("input_a has {:?}", input_a);
    println!("input_e has {:?}", input_e);
    input_a + input_b
}

struct GenStruct<T, U> {
    gen_t: T,
    gen_u: U
}

impl<T, U> GenStruct<T, U> 
where T: std::fmt::Debug,
      U: std::fmt::Debug {
          fn log_something(&self) {
              println!("{:?} {:?}", self.gen_t, self.gen_u);
          }
      }
