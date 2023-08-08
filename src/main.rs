mod utils;

use utils::{
    get_i64,
};

fn prefill_vector(prefill: &mut Vec<i64>, n: i64) {
    let mut i = 2;
    while prefill.len() <= (n as usize) {
        let val: i64 = prefill[i - 2] + prefill[ i - 1];
        prefill.push(val);
        i += 1
    }
}

fn fibonacci_prefill(n: i64) -> i64 {
    let mut prefill: Vec<i64> = vec![0, 1];
    prefill_vector(&mut prefill, n);
    prefill[n as usize]
}

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

fn fibonacci_bottom_up(n: i64) -> i64 {
    if n < 2 { return n; }

    let (mut fib_less_2, mut fib_less_1) = (0, 1);
    let mut fib = 0;
    for _ in 2..=n {
        fib = fib_less_2 + fib_less_1;
        fib_less_2 = fib_less_1;
        fib_less_1 = fib;
    }
    fib
}

fn test_fibonacci_on_the_fly() {
    println!("Testing fibonacci_on_the_fly");
    let fibs: Vec<i64> = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
    for (i, f) in fibs.iter().enumerate() {
        assert!(*f == fibonacci(i as i64));
    }

    let n = get_i64("Enter fibonacci number to calculate: ");
    let val = fibonacci(n);
    println!("Fibonacci number {n} = {val}");
}

fn test_fibonacci_prefill() {
    println!("Testing fibonacci_prefill");
    let fibs: Vec<i64> = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
    for (i, f) in fibs.iter().enumerate() {
        assert!(*f == fibonacci_prefill(i as i64));
    }

    let n = get_i64("Enter fibonacci number to calculate: ");
    let val = fibonacci_prefill(n);
    println!("Fibonacci number {n} = {val}");
}

fn test_fibonacci_bottom_up() {
    println!("Testing fibonacci_bottom_up");
    let fibs: Vec<i64> = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
    for (i, f) in fibs.iter().enumerate() {
        assert!(*f == fibonacci_bottom_up(i as i64));
    }

    let n = get_i64("Enter fibonacci number to calculate: ");
    let val = fibonacci_bottom_up(n);
    println!("Fibonacci number {n} = {val}");
}

fn main() {
   test_fibonacci_on_the_fly(); 
   test_fibonacci_prefill(); 
   test_fibonacci_bottom_up(); 
}
