

//mod SecondModule;
// call other local mod
//use crate::SecondModule::implSecondModule;
use  crate::SecandModule::implSecondModule;
pub fn say_hello(){
    println!("i am karima");
}
pub fn mod_to_mod_function() {
    println!("hey!! from other mod {}", implSecondModule::mod_to_mod());
}


