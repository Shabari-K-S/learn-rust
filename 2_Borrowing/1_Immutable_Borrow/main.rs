fn calculate_length(s: &String) -> usize {
    s.len()
}  // Here, `s` goes  out of scope , but since it doesn't own the data, nothing is disallocated


fn main() {
    let s1 = String::from("code");
    let len = calculate_length(&s1);    // we pass a reference (&)

    println!("Length of the word `{s1}` is {len}"); // s1 is still valid
}
