fn main() {
    println!("-----------Vec ------------");
    let mut v = Vec::<u32>::with_capacity(5);
    v.extend(1..10);
    let boxes: Vec<Box<&u32>> = (&v).into_iter().map(|a| Box::new(a)).collect();
    let x = v[0];
    let b = &boxes[0];
    let vclone = (&v).clone();
    {
        for (i, j) in (&v).into_iter().enumerate() {
            println!("{} = {}", i, j);
        }
    }
    println!("---------");
    let mut slice_ref = &(&v)[..];
    for (i, j) in slice_ref.into_iter().enumerate() {
        println!("{} = {}", i, j);
    }
    println!("first is {}", slice_ref.first().unwrap_or(&0u32));
    println!("last is {}", slice_ref.last().unwrap_or(&0u32));
    println!("number 4  is {}", slice_ref.get(4).unwrap_or(&0u32));

    v.push(77u32);

    for (i, j) in v.into_iter().enumerate() {
        println!("{} = {}", i, j);
    }
    /*   v.pop();

    for (i, j) in v.into_iter().enumerate() {
        println!("{} = {}", i, j);
    } */

    let table = [[1, 2], [3, 4], [5, 6], [7, 8]].join(&60);
    for (i, j) in (&table).into_iter().enumerate() {
        println!("{} = {}", i, j);
    }

    use std::collections::VecDeque;
    let mut vector: VecDeque<u32> = VecDeque::new();

    vector.push_front(10u32);

    use std::collections::LinkedList;

    let mut list1 = LinkedList::new();
    list1.push_back('a');

    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();

    // Wrap values in `Reverse`
    heap.push(Reverse(1));
    heap.push(Reverse(5));
    heap.push(Reverse(2));

    use std::collections::BTreeMap;
    use std::collections::BTreeSet;
    use std::collections::HashMap;
    use std::collections::HashSet;
    //get mut to make a modification
    // len , is_empty , capacity ,
    println!("---------------------");
    let mut vec4 = vec![10, 10, 10, 9, 20];

    vec4.dedup_by_key(|i| *i);

    println!("---------------------");

    for i in vec4.into_iter() {
        println!("{} ", i);
    }
    println!("-----concat----------------");
    let table = [[1, 2], [3, 4], [5, 6], [7, 8]].concat();
    for (i, j) in (&table).into_iter().enumerate() {
        println!("{} = {}", i, j);
    }

    println!("-----Splitting----------------");

    println!("split_at");
    let v = [1, 2, 3, 4, 5, 6];

    {
        let (left, right) = v.split_at(0);
        assert_eq!(left, []);
        assert_eq!(right, [1, 2, 3, 4, 5, 6]);
    }
    {
        let (left, right) = v.split_at(2);
        assert_eq!(left, [1, 2]);
        assert_eq!(right, [3, 4, 5, 6]);
    }
    {
        let (left, right) = v.split_at(6);
        assert_eq!(left, [1, 2, 3, 4, 5, 6]);
        assert_eq!(right, []);
    }

    println!("split_first");
    let x = &[0, 1, 2];

    if let Some((first, elements)) = x.split_first() {
        println!("{}  ", first);
        for (i, j) in elements.into_iter().enumerate() {
            println!("{} = {}", i, j);
        }
    }
    println!("split_last");

    if let Some((first, elements)) = x.split_last() {
        println!("{}  ", first);
        for (i, j) in elements.into_iter().enumerate() {
            println!("{} = {}", i, j);
        }
    }
    println!("slice.split");

    let slice = [10, 40, 33, 44, 55, 77, 20];
    let mut iter = slice.splitn(3, |num| num == &40u32);

    for j in iter.into_iter() {
        println!("***********");
        for k in j {
            println!("{} ", k);
        }
    }

    println!("slice.windows");
    let mut iter = slice.windows(3);

    for j in iter.into_iter() {
        println!("***********");
        for k in j {
            println!("{} ", k);
        }
    }
}
