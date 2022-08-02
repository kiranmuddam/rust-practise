fn is_even(num: i64) -> bool {
    /*  This function takes a i64 integer and checks
        Checks where it is even by using modulus operator.
        If it is even, it will return true or it will return false.
     */
    if num % 2 == 0 {
        return true;
    }

    return false;
}

fn main() {
    // This prints to console where the given number is even or not
    println!("Is number {} is even?: {}", 100, is_even(100));
}