fn main() {
    println!("Hello, world!");
}

fn print_even() {
    for x in 0..10 {
        if x % 2 == 0 {
            println!("{}", x);
        }
    }
}

fn sum_squares() -> u32 {
    let mut sum = 0;
    for x in 0..10 {
        sum += x * x;
    }

    sum
}

fn square_vector(numbers: &mut Vec<u32>) {
    // Exercise: change to `for x in numbers.iter_mut()`
    for i in 0..numbers.len() {
        numbers[i] *= numbers[i];
    }
}
