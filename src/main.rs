mod utils;

use utils::{
    get_i64,
};


fn fibonacci_on_the_fly(dp_array: &mut Vec<i64>, n: i64) -> i64 {
    if dp_array[n as usize] == -1i64 {
        let first = fibonacci_on_the_fly(dp_array, n - 2);
        let second = fibonacci_on_the_fly(dp_array, n - 1);
        dp_array[n as usize] = first + second
    }
    dp_array[n as usize]
}

fn fibonacci(n: i64) -> i64 {
    let mut dp_array = vec![0, 1];
    for _i in 2..=n {
        dp_array.push(-1i64);
    }
    fibonacci_on_the_fly(&mut dp_array, n)
}

fn main() {
    let fibs: Vec<i64> = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
    for (i, f) in fibs.iter().enumerate() {
        assert!(*f == fibonacci(i as i64));
    }

    let n = get_i64("Enter fibonacci number to calculate: ");
    let val = fibonacci(n);
    println!("Fibonacci number {n} = {val}");
}
