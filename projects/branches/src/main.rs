fn main() {
    let number = 8;

    if number % 4 == 0 {
        println!("The number is divisible by 4.");
    } else if number % 3 == 0 {
        println!("The number is divisible by 3");
    } else if number % 2 == 0 {
        println!("The number is divisible by 2");
    } else {
        println!("The number is not divisible by 2, 3, or 4.");
    }
}
