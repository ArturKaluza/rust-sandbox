mod print;
mod vars;
mod types;
mod string;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod pointer_ref;

fn main() {
    print::run();
    vars::run();
    types::run();
    string::run();
    tuples::run();
    arrays::run();
    vectors::run();
    conditionals::run();
    loops::run();
    functions::run();
    pointer_ref::run();
}
