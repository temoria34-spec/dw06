/// Counts from 1 to 10, printing each value, and returns the final count.
pub fn counter() -> i32 {
    let mut count = 1;

    while count <= 10 {
        println!("{}...", count);
        count += 1;
    }

    count
}
