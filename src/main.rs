fn main() {
    let count = 0;

    println!("1. count = {}", count);
    
    //mutable
    let mut count = 1;

    count += 1;

    println!("2. {0} = {1:#?}", "count", count);
    // # mean pretty print.
    // ? mean debug.

    if count == 2 {
        println!("3. count = {count}");
    }

    assert_eq!(count, 2);

    println!("4. count = {count} = 0x{count:x}");

    println!("Hello, world!");
}
