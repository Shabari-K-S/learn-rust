# Borrowing at the Code Level: References

## Immutable Borrow (`&amp;T`)

Allows reading data without taking ownership

## Mutable Borrow (`&amp;mut T`)

Allows changing the data in place without taking ownership. The variable itself must be declared with `mut`


## The Core Rule the Compiler Enforces

At any given point in your code, you can have **either**:

1. Any number of immutable references (`&amp;T`), **OR**
2. Exactly one mutable reference (`&amp;mut T`).

```rust
fn main() {
    let mut data = String::from("rust");

    let r1 = &amp;data;     // OK: immutable borrow
    let r2 = &amp;data;     // OK: multiple readers allowed
    // let r3 = &amp;mut data; 
    // ^ COMPILE ERROR: cannot borrow `data` as mutable because it is also borrowed as immutable!

    println!("{r1} and {r2}");
}
```
