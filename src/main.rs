use num_bigint::BigInt;
use num_format::*;
use num_traits::{One, Zero};
use ordinal::Ordinal;
use std::mem::replace;
use std::{collections::HashMap, hash::Hash, io};

// Calculate large fibonacci numbers.
fn fib(n: u128) -> BigInt {
    let mut f0: BigInt = Zero::zero();
    let mut f1: BigInt = One::one();

    for _ in 0..n {
        let f2 = f0 + &f1;
        // This is a low cost way of swapping f0 with f1 and f1 with f2.
        f0 = replace(&mut f1, f2);
    }
    f0
}


fn memoize<A, R, F>(cache: &mut HashMap<A, R>, func: F, arg: A) -> R
where
    A: Eq + Hash + Clone,
    R: Clone,
    F: Fn(&mut HashMap<A, R>, A) -> R,
{
    match cache.get(&arg).map(|x| x.clone()) {
        Some(result) => result,
        None => {
            let result = func(cache, arg.clone());
            cache.insert(arg, result.clone());
            result
        }
    }
}

fn fib_memo(cache: &mut HashMap<u128, u128>, arg: u128) -> u128 {
    match arg {
        0 => 0,
        1 => 1,
        n => memoize(cache, fib_memo, n - 1) + memoize(cache, fib_memo, arg - 2),
    }
}

fn main() {
    println!("Fibonacci generator");

    loop {
        let mut str_input = String::new();

        println!("\nType \"quit\" to end the program or");

        println!("Enter a positive integer >= 0");

        io::stdin()
            .read_line(&mut str_input)
            .expect("Failed to reade line");

        if str_input.trim().to_string().to_lowercase() == "quit" {
            break;
        }

        let num: u128 = match str_input.trim().parse::<u128>() {
            Ok(n) => match n {
                0..=186 => n as u128,
                _ => {
                    println!(
                        "The {fib} Fibonacci number is: \n{:0}",
                        fib(n),
                        fib = Ordinal(n).to_string()
                    );
                    continue;
                }
            },
            Err(_) => continue,
        };

        println!(
            "The {} Fibonacci number is: \n{}",
            Ordinal(num).to_string(),
            fib_memo(&mut HashMap::new(), num).to_formatted_string(&Locale::en)
        );
    }
}
