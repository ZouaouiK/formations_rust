pub fn var_muta(){
    let mut _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error!
    //si _immutable_binding not mut 

    //ok 
    _immutable_binding += 4;
    println!("After mutation: {}", _immutable_binding);
    // FIXME ^ Comment out this line
}