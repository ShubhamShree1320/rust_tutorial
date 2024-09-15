mod enum_method;
mod string_function;
mod struct_method;
fn main() {
    let rect_var = struct_method::Rect {
        width: 10,
        height: 20,
    };
    println!("the area of rectangle is = {}", rect_var.area());
    let shape = enum_method::Shape::Square(10.0);
    println!("the area of shape is = {}", shape.area());
    let name = String::from("hello");
    string_function::string_function(&name);
    println!("Hello, world!");
    let num = 4;
    println!("{}", is_even(&num));
    println!(
        "fibonacci number for target = {} is = {}",
        num,
        fibonacci(&num)
    )
}
fn fibonacci(target: &i8) -> i32 {
    let mut first = 0;
    let mut second = 1;
    if *target == 0 {
        return first;
    }
    if *target == 1 {
        return 1;
    }
    for __ in 1..*target - 1 {
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
