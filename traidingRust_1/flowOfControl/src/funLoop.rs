pub fn loop_function() {
    //move convertit toutes les variables capturées par référence ou référence mutable en variables capturées par valeur.

    let mut count = 0u32;
    println!("Let's count until infinity!");
    // Infinite loop
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            // Skip the rest of this iteration
            continue;
        };
        println!("xx {}", count);
        if count == 5 {
            println!("OK, that's enough");
            // Exit this loop
            break;
        }
    }
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!(" Counter {}", result);
    assert_eq!(result, 20);
}

pub fn function_while(){
        // A counter variable
        let mut a = 10;
        let b = 11;
        let mut n = 1;
        while a < b {
              println!("{}", a);
            a += 1;
        }    
        // Loop while `n` is less than 101
        while n < 11 {
            if n % 15 == 0 {
                println!("fizzbuzz");
            } else if n % 3 == 0 {
                println!("fizz");
            } else if n % 5 == 0 {
                println!("buzz");
            } else {
                println!("{}", n);
            }
            // Increment counter
            n += 1;
        }
}
pub fn funcFor(){
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("karima");
        } else if n % 3 == 0 {
            println!("koukou");
        } else if n % 5 == 0 {
            println!("zouaoui");
        } else {
            println!("{}", n);
        }
    }
}
// into_iter, iter et iter_mut:: gèrent tous la conversion d'une collection
// en un itérateur de différentes manières,
// en fournissant différentes vues sur les données qu'il contient.
pub fn iter_function(){
    let names = vec!["karima", "zouaoui", "koukou"];

    for name in names.iter() {
        match name {
            &"koukou" => println!("i am karima zouaoui!"),
            // TODO ^ Try deleting the & and matching just "koukou"
            _ => println!("Hello {}", name),
        }
    }
    
    println!("names: {:?}", names);
}