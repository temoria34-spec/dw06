/// Function that prints the numbers from 1 to 10
///
/// Returns the final value from the count variable.
///
/// # Examples
///
/// ```
/// let result = dw06::counter();
/// assert_eq!(result, 10);
/// ```
pub fn counter() -> i32 {
    let mut count = 1;

    while count <= 10 {
        println!("Count: {}", count);
        count += 1;   // THIS is the missing increment
    }

    count
}


