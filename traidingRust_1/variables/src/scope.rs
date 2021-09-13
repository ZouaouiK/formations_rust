pub fn var_scope(){
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

       
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

 
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);

    println!("---------Declare first ------------");
    // Declare a variable binding
    let a_binding;

    {
        let x = 2;

        // Initialize the binding
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let  mut another_binding=1;

    // Error! Use of uninitialized binding
    println!("another binding: {}", another_binding);
    // FIXME ^ Comment out this line

    another_binding = 1;

    println!("another binding: {}", another_binding);
}