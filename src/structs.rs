struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct Color2(u8, u8, u8);

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Self {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let c2 = Color2(255, 0, 0);

    println!("Color: {} {} {}", c2.0, c2.1, c2.2);

    let mut p = Person::new("John", "Doe");
    println!("Person {} {}",p.first_name, p.last_name);
    println!("full name {}", p.full_name());

    p.set_last_name("Bravo");
    println!("full name {}", p.full_name());
    println!("Person tuple {:?}", p.to_tuple());

    let person = Person::new("Duglas", "Malford");
    let person2 = Person {
        first_name: "Dug".to_string(),
        ..person
    };

    println!("{} {}", person2.first_name, person2.last_name);
    // derive(Debug)
    println!("{:?}", person2);

    let random = RandomInfo {
        some_bool: true,
        some_int: 28,
    };

    print_if_is_valid(&random);
}

struct RandomInfo {
    some_bool: bool,
    some_int: i64,
}


impl RandomInfo {
    fn new(a: bool, b: i64) -> Self {
        Self {
            some_bool: a,
            some_int: b
        }
    }

    // if you want to use "dot" notation the first param should be &self
    fn is_smaller(&self, compare_to: i64) -> bool {
        self.some_int < compare_to
    }
}

trait SomeTrait {
    fn is_valid(&self) -> bool;
}

// polymorphism 
impl SomeTrait for RandomInfo {
    fn is_valid(&self) -> bool {
        self.some_bool
    }
}

fn print_if_is_valid(check_me: &dyn SomeTrait) {
    if check_me.is_valid() {
        println!("Yay!");
    }
}

impl Default for RandomInfo {
    fn default() -> Self {
        Self {
            some_bool: false,
            some_int: 0
        }
    }
}

// empty struct to group some functionality
struct NoFields{}

// generic struct
struct Pair<T> { x: T, y: T}

// struct as tuple
struct Colors(u8, u8, u8);