
mod FirstModule;
mod SecandModule;
mod Multiple_module;
use FirstModule::implFistModule::{say_hello,mod_to_mod_function};



fn main() {
    say_hello();
    mod_to_mod_function();
    println!("---");
	Multiple_module::mod1::foo();
	Multiple_module::mod2::foo();
}
