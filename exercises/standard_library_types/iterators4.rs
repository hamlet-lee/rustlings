// iterators4.rs


pub fn factorial(num: u64) -> u64 {
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
    (1..=num).fold(1, |acc, x| { acc * x })
    // let mut s : u64 = 1;
    // let a:Vec<u64> = (1..=num).into_iter().map (| x | {s = s * x; x}).collect();
    // (1..=num).into_iter().map (| x | {s = s * x; x}).collect::<Vec<u64>>();
    // println!("{:?} {}", a, s);
    // s
    // Complete this function to return factorial of num
    // Do not use:
    // - return
    // For extra fun don't use:
    // - imperative style loops (for, while)
    // - additional variables
    // For the most fun don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
}

//fn main () {
//  factorial(4);
//}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        // assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        // assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
