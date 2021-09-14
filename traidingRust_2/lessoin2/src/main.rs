pub trait Wallet<T, U>
where
    T: Clone + Sized + Copy,
    U: Clone,
{
    type ReturnType;
    type ReturnType2;

    fn show_wallet(&self) -> Self::ReturnType;
}

impl Wallet<u32, u32> for u32 {
    type ReturnType = u64;
    type ReturnType2 = u64;
    fn show_wallet(&self) -> u64 {
        10u64
    }
}
fn main() {
    println!("Hello, world!");
    let x = 10u32;
    //let y=x.show_wallet();
    let y;
    y = Wallet::show_wallet(&x);
    println!("{}", y);

    println!("--------iterator-------------");
    let rust_group = vec!["Bacem", "karima", "amal", "khoukha", "jawaher"];
    let other_group = vec!["Bacem", "karima", "amal", "khoukha", "jawaher"];
    for membre in &rust_group {
        println!("{}", membre);
    }
    println!("--------into  iter-------------");
    let mut iter = (&rust_group).into_iter();
    let value = iter.next();
    let real_value = match value {
        Some(v) => v,
        None => "",
    };
    println!("{}", real_value);

    while let Some(v) = iter.next() {
        println!("{}", v)
    }
    println!("--------tchabrich -------------");
    let t = 1..10; //iterator
    for membre in t {
        println!("{}", membre);
    }
    println!("-------- ***** Adaptator **** -------------");
    println!("--------adaptator : Map-------------");

    let y = rust_group.iter().map(|m| (String::new() + m + "narjassi"));
    for membre in y {
        println!("{}", membre);
    }
    println!("--------adaptator : filter-------------");

    let y = rust_group.iter().filter(|x| x.len() <= 6);
    for membre in y {
        println!("{}", membre);
    }
    println!("--------adaptator : filter map-------------");

    let z = rust_group.iter().filter_map(|m| {
        if m.len() > 5 {
            return Some(String::new() + m + " khoukha ");
        }
        None
    });
    for member in z {
        println!("{}", member);
    }
    println!("--------adaptator : flat  map-------------");

    let z = rust_group.iter().flat_map(|m| {
        if m.len() > 5 {
            return Some([m, "karima"]);
        }
        None
    });
    for member in z {
        for m1 in member {
            println!("{}", m1);
        }
    }
    println!("--------collect-------------");
    let it: Vec<u32> = (1..10).collect();

    println!("--------adaptator : scan-------------");
    let iter = rust_group
        .iter()
        .scan("Sqoin membre are ".to_string(), |s, item| {
            s.push_str(item);
            s.push_str(" ");
            Some(s.clone())
        });
    for member in iter {
        println!("{} ", member);
    }
    println!("--------adaptator : take-------------");
    let y = rust_group.iter().take(3);
    for membre in y {
        println!("{}", membre);
    }
    println!("--------adaptator : take while-------------");
    let y = rust_group.iter().take_while(|m| m.len() > 4);
    for membre in y {
        println!("{}", membre);
    }
    println!("--------adaptator : skip-------------");
    let y = rust_group.iter().skip(3);
    for membre in y {
        println!("{}", membre);
    }
    println!("--------adaptator : skip while-------------");
    let y = rust_group.iter().skip_while(|m| m.len() > 4);
    for membre in y {
        println!("{}", membre);
    }
    println!("--------adaptator : peekable-------------");
    let mut x = rust_group.iter().peekable();
    if let Some(r) = x.peek() {
        println!("{}", r);
    }
    if let Some(r) = x.next() {
        println!("{}", r);
    }
    if let Some(r) = x.peek() {
        println!("{}", r);
    }
    println!("--------adaptator : Fuse-------------");
    let mut iter = (&rust_group).into_iter().fuse();
    while let Some(v) = iter.next() {
        println!("{}", v);
    }
    let x = iter.next();
    if let Some(r) = x {
        println!("{}", r);
    } else {
        println!("{}", "None");
    }
    println!("--------adaptator : Reverse-------------");
    let mut iter = (&rust_group).into_iter();
    while let Some(v) = iter.next_back() {
        println!("{}", v);
    }
    println!("--------adaptator :Reverse cas 2-------------");
    let mut iter = (&rust_group).into_iter().rev();
    while let Some(v) = iter.next() {
        println!("{}", v);
    }
    println!("--------adaptator : Inspect-------------");
    let mut iter = (&rust_group).into_iter().rev();
    //iter.inspect(|m| println!(" inspection {} ",m)).;
    "bacem"
        .chars()
        .inspect(|m| println!(" inspection {} ", m))
        .flat_map((|c| c.to_uppercase()))
        .inspect(|m| println!(" inspection {} ", m));
    println!("--------adaptator : chain-------------");
    let mut chain = (&rust_group).iter().chain(other_group.iter());
    while let Some(v) = chain.next() {
        println!("{}", v);
    }
    println!("--------adaptator : enumerate -------------");
    for (i, m) in (&rust_group).into_iter().enumerate() {
        println!("{} {} ", i, m);
    }
    println!("--------adaptator : zip -------------");
    let mut zip = (&rust_group).iter().zip(other_group.iter());
    while let Some(v) = zip.next() {
        println!("{} {}", v.0, v.1);
    }
    println!("-------- by ref  -------------");
    for i in (&rust_group).into_iter().by_ref() {
        println!("{} ", i);
    }
    println!("--------  adaptator : cloned -------------");
    for i in (&rust_group).into_iter().cloned() {
        println!("{} ", i);
    }
    println!("--------  adaptator : cycle -------------");
    for (i, m) in (&rust_group).into_iter().cycle().enumerate() {
        println!("{} {} ", i, m);
        if i > 10 {
            break;
        }
    }
    println!("-------- ***** consuming iterators **** -------------");
    let mut numbers: Vec<u32> = vec![1, 2, 3, 7, 10];
    let numbe_riter = (&numbers).into_iter();
    println!("sum = {}", numbe_riter.sum::<u32>());
    let numbe_riter = (&numbers).into_iter();
    println!("product = {}", numbe_riter.product::<u32>());
    let numbe_riter = (&numbers).into_iter();
    println!("count = {}", numbe_riter.count());
    let numbe_riter = (&numbers).into_iter();
    //println!("min = {:?}",numbe_riter.min());
    //ou bien
    println!("min = {}", numbe_riter.min().unwrap());
    let numbe_riter = (&numbers).into_iter();
    println!("max = {:?}", numbe_riter.max().unwrap());

    println!("---------any all ");
    let mut numbe_riter = (&numbers).into_iter();
    println!("any = {}", numbe_riter.any(|m| m > &8u32));
    let mut numbe_riter = (&numbers).into_iter();
    println!("all = {}", numbe_riter.all(|m| m > &8u32));
    println!("---------position -------- ");
    let mut numbe_riter = (&numbers).into_iter();
    println!(
        "position de 7= {}",
        numbe_riter.position(|m| m > &8u32).unwrap_or(0)
    );
    let mut numbe_riter = (&numbers).into_iter();
    println!("fold de  {}", numbe_riter.fold(0, |x, y| x + y));

    let mut numbe_riter = (&numbers).into_iter();
    println!(" nth   {}", numbe_riter.nth(2).unwrap_or(&2u32));


    let mut numbe_riter = (&numbers).into_iter();
    println!(" last   {}", numbe_riter.last().unwrap_or(&2u32));


    let mut numbe_riter = (&numbers).into_iter();
    println!(" find   {}", numbe_riter.find(|m| m > &&5u32).unwrap_or(&2u32));


    println!("************ Extend *************");

    numbers.extend(0..5);

    for i in &numbers {
        println!("{} " , i);
    }
    println!("************ Partition *************");

    let mut chain  = rust_group.iter().chain(other_group.iter());

    let (p1,p2) : (Vec::<&str> ,Vec::<&str> )= chain.partition(|n| n.len() <= 4);

    for i in p1 {
        println!(" partition 1 {}" , i);
    }

    for i in p2 {
        println!(" partition 2 {}" , i);
    }
}


fn map_membre(m: &&str) {
    println!("{}", m);
}
