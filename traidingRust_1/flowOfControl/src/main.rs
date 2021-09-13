mod functionIf;
mod funLoop;
mod matchFunction;
fn main() {
    println!("Hello, world!");
    println!("---------- if/else----------");
    functionIf::if_else();
    println!("---------- Loop ----------");
    funLoop::loop_function();
    println!("---------- while ----------");
    funLoop::function_while();
    println!("---------- for and range ----------");
    funLoop::funcFor();
    println!("---------- iter and iter_mut and into_iter ----------");
    funLoop::iter_function();
    println!("---------- match ----------");
    matchFunction::match_function();
    println!("---------- match and enum ----------");
    matchFunction::match_function1();
}
