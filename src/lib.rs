fn main() {
    let n = 10;

    let mut sum_of_squares = 0;
    let mut sum = 0;

    for i in 1..=n {
        sum_of_squares += i * i;
        sum += i;
    }

    let square_of_sum = sum * sum;
    let difference = square_of_sum - sum_of_squares;

    println!("Difference for the first {} natural numbers: {}", n, difference);
}
