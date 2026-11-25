fn main() {
    let s1 = String::from("hello world"); // s1 is the owner of "hello world"
    let s2 = s1;                         // now the ownership is moved from s1 to s2

    // println!("{s1}");
    //            ^ COMPILE ERROR: borrow of moved value: `s1`

    println!("{s2}");
}
