fn main() {
    println!("----------- Split At ------------");
    // Slicing a Vec
    let vec = vec![1, 2, 3, 5, 9, 90];
    let int_slice = &vec[0..4];
    for i in int_slice.iter() {
        println!("{}", i)
    }
    println!("---------------------------");
    let mut x = [1, 2, 3, 6, 7, 8, 9, 77];
    let x = &mut x[..]; // Take a full slice of `x`.
    x[1] = 7;
    println!(" length x is ={} ", x.len());
    println!("empty x is = {}", x.is_empty());
    for i in x.iter_mut() {
        println!("{}", i);
    }
    println!("------- First and first_mut ---------");
    // Returns the first element of the slice, or None if it is empty.
    if let Some(&first) = int_slice.first() {
        println!("first {} ", first);
    }
    let x = &mut [90, 1, 2, 9, 6];
    //Returns a mutable pointer to the first element of the slice, or None if it is empty.
    if let Some(first) = x.first_mut() {
        println!("first mut {} ", first);
    }
    println!("--------- Split First And Split First Mut -----------");
    // Returns the first and all the rest of the elements of the slice, or None if it is empty.
    if let Some((first, elements)) = int_slice.split_first() {
        println!("{}", first);
        for (i, j) in elements.into_iter().enumerate() {
            println!("split first ={}:{}", i, j);
        }
    }
    println!("------------");
    //Returns the first and all the rest of the elements of the slice, or None if it is empty.
    if let Some((first, elements)) = x.split_first_mut() {
        println!("{}", first);
        for (i, j) in elements.into_iter().enumerate() {
            println!("split first mut ={}:{}", i, j);
        }
    }

    println!("---------- Split Last And Split Last Mut -----------");
    //Returns the last and all the rest of the elements of the slice, or None if it is empty.
    if let Some((last, elements)) = int_slice.split_last() {
        println!("{}", last);
        for (i, j) in elements.into_iter().enumerate() {
            println!("Split Last mut ={}:{}", i, j);
        }
    }
    // Returns the last and all the rest of the elements of the slice, or None if it is empty.
    if let Some((last, elements)) = x.split_last_mut() {
        println!("{}", last);
        for (i, j) in elements.into_iter().enumerate() {
            println!("Split Last Mut ={}:{}", i, j);
        }
    }
    println!("-----------last , last mut , get and get mut-----------");
    // Returns the last element of the slice, or None if it is empty.
    if let Some(last) = int_slice.last() {
        println!("last {}", last);
    }
    if let Some(last) = x.last_mut() {
        println!("last mut {}", last);
    }
    if let Some(elem) = int_slice.get(2) {
        println!("get element {}", elem);
    }
    if let Some(elem) = x.get_mut(2) {
        println!("get mut element {}", elem);
    }
    println!("-----------Windows-----------");
    let v = ["a", "b", "c", "d"];
    let iter = v.windows(2);
    for j in iter.into_iter() {
        println!("-----");
        for k in j {
            println!("{} ", k);
        }
    }
    println!("-----------Swaps A -> B -----------");
    //Swaps two elements in the slice.
    let mut v = ["a", "b", "c", "d"];
    v.swap(1, 3);
    for i in v.iter() {
        println!("after swap ={}", i);
    }
    println!("-----------Reverse -----------");
    //Reverses the order of elements in the slice, in place.
    v.reverse();
    for i in v.iter() {
        println!("after reverse ={}", i);
    }
    println!("-----------chunks -----------");
    let iter = v.chunks(2);
    for i in iter.into_iter() {
        println!("after chunks  ={:?}", i);
    }

    println!("--------- concat-----------");
    let table = [[1, 2], [3, 4], [5, 6], [7, 8]].concat();
    for (i, j) in (&table).into_iter().enumerate() {
        println!("{} : {}", i, j);
    }
    println!("--------- join -----------");
    let table = [[1, 2], [3, 4], [5, 6], [7, 8]].join(&111);
    for (i, j) in (&table).into_iter().enumerate() {
        println!("{} : {}", i, j);
    }
    println!("--------- rsplitn -----------");
    let v = [10, 40, 30, 20, 60, 50];
    for group in v.rsplitn(2,|num| *num == 20) {
        println!("{:?}", group);
    }
}
