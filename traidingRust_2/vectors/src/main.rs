fn main() {
    println!("*********** Vectors ************");
    println!("------------ Create empty vector------------");
    let mut a = Vec::<u32>::new(); //1.With new() keyword
    let mut b: Vec<u32> = vec![]; //2.Using the vec! macr
    println!("------------ Create with data types------------");
    let mut a2: Vec<i32> = Vec::new();
    let mut b2: Vec<i32> = vec![];
    let mut b3 = vec![1i32, 2, 3]; //Sufixing 1st value with data type

    let mut b4 = vec![1, 2, 3];
    let mut b5: Vec<i32> = vec![1, 2, 3];
    let mut b6 = vec![1i32, 2, 3];
    let mut b7 = vec![0; 10]; //Ten zeroes
    println!("------------ Access and change data------------");
    //Accessing and changing existing data
    let mut c = vec![5, 4, 3, 2, 1];
    c[0] = 1;
    c[1] = 2;
    //c[6] = 2; Cannot assign values this way, index out of bounds
    println!("c {:?}", c); //[1, 2, 3, 2, 1]

    //push and pop
    let mut d: Vec<i32> = Vec::new();
    d.push(1); //[1] : Add an element to the end
    d.push(2); //[1, 2]
    d.push(3); //[1, 2,3]
    d.push(0); //[1, 2,3]
    d.extend(1..4);
    d.pop(); //[1] : : Remove an element from the end
    println!("d {:?}", d);
    // 🔎 Capacity and reallocation
    let mut e: Vec<i32> = Vec::with_capacity(10);
    println!("Length: {}, Capacity : {}", e.len(), e.capacity()); //Length: 0, Capacity : 10

    // These are all done without reallocating...
    for i in 0..10 {
        e.push(i);
    }
    // ...but this may make the vector reallocate
    e.push(11);
    println!("e {:?}", e);
    println!("------------ reference and mut------------");
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("A reference to {}", i);
    }

    for i in &mut v {
        println!("A mutable reference to {}", i);
    }

    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }
}
