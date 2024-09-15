mod string_function;
fn main() {
    string_function::string_function();
    println!("Hello, world!");
    let num = 24;
    println!("{}", is_even(&num));
    println!("{}", num);
    println!(
        "fibonacci number for target = {} is = {}",
        num,
        fibonacci(num)
    )
}
fn fibonacci(target: i8) -> i32 {
    let mut first = 0;
    let mut second = 1;
    if target == 0 {
        return first;
    }
    if target == 1 {
        return 1;
    }
    for __ in 1..target - 2 {
        let tmp = second;
        second = second + first;
        first = tmp;
    }
    return second;
}
fn is_even(number: &i8) -> i8 {
    if number % 2 == 0 {
        return number / 2;
    }
    return -1;
}
