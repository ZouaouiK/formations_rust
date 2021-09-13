pub fn cast_copy_var(){
    let decimal = 65.4321_f32;

    // Error! No implicit conversion
    // let integer: u8 = decimal;
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);
      // Unless it already fits, of course.
      println!(" 128 as a i16 is: {}", 128 as i16);
      // 128 as u8 -> 128, whose two's complement in eight bits is:
    
let x = 100u32 + (10u8 as u32);

    let y: u32 = 1000u32;

    let f: f32 = 0.001f32;

    let b: bool = {
        let mut ret = false;
        if x < y {
            ret = true;
        }
        ret
    };

    let func = move |z: u32| {
        let mut s = x;
        s = s * z * z;
        s
    };
    println!(" value of x is {}", x);
    println!(" value of y is {}", y);
    println!(" value of f is {}", f);
    println!(" value of b is {}", b);
    println!(" value of func(y) is {}", func(y));

}