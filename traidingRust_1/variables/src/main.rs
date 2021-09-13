mod var;
mod cast;
mod mutability;
mod scope;
fn main() {
    println!("------------copy--------------");
    var::copy_var();
    println!("------------Cast And copy--------------");
    cast::cast_copy_var();
    println!("------------Mutability--------------");
    mutability::var_muta();
    println!("------------Scope and Shadowing--------------");
    scope::var_scope()
}
