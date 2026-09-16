
/// Function that prints the numbers from 1 to 10
///
/// Returns the final value from the count variable.
///
/// # Examples
///
/// ```
/// let result = dw06::counter();
/// assert_eq!(result,10);
/// ```
pub fn counter() -> i32 {
    let mut count = 1;

    // Counts to 10 and prints each value. Increases value
    // by 1 each loop
    while count <= 10 {
        count + 1;
        println!("Count: {}", count)
    }

    count
}

