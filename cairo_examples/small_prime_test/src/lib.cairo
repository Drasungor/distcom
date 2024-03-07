fn main() -> felt252 {

    let tested_number = 77_u128;
    let mut counter = 3_u128;
    let mut is_possibly_prime = true;
    let mut failure_value = 0;

    while ((counter != tested_number) && (is_possibly_prime)) {
        if (tested_number % counter == 0) {
            is_possibly_prime = false;
            failure_value = counter;
        }
        counter += 2;
    };
    if (!is_possibly_prime) {
        println!("The number {} is not prime, it is divisible by {}", tested_number, failure_value);
    } else {
        println!("The number {} is prime", tested_number);
    }
    5
}