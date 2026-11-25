fn add_well(s: &mut String) {
    s.push_str(" Well");
}

fn main() {
    let mut s1 = String::from("Code");
    add_well(&mut s1);

    println!("{s1}");
}
