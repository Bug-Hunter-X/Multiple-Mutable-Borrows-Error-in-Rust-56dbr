fn main() {
    let mut x = 5;
    { // Create a scope for mutable borrow of x
        let y = &mut x;
        *y = 10;
    }
    let z = &mut x; // This will no longer cause an error
    *z = 15;
    println!("x = {}", x);
}