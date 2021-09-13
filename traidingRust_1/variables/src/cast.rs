pub fn cast_copy_var(){
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