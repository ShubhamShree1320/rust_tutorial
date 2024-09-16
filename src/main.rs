mod enum_method;
mod string_function;
mod struct_method;
mod vector_method;
use std::collections::HashMap;
use std::fs::read_to_string;
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
    );
    let result_txt = read_to_string("src/my_file.txt");
    match result_txt {
        Ok(content) => println!("{}", content),
        Err(error) => println!("{}", error),
    };
    /*      I am going to write a vector function that takes vector as input and reurns vector with
     *      even values    */
    let mut vec1 = Vec::new();
    for i in 1..10 {
        vec1.push(i);
    }
    let vec2 = get_even(&vec1);
    println!("{:?}", vec2);
    /*I am going to implement hashmap*/

    let mut hm = HashMap::new();
    hm.insert("Shubham", 1);
    hm.insert("Rishabh", 2);
    hm.insert("Lav", 3);

    let value_required = hm.get("Shubham");
    match value_required {
        Some(x) => println!("{}", x),
        None => println!("Not found"),
    }

    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2: Vec<i32> = vec1
        .iter()
        .filter(|x| *x % 2 != 0)
        .map(|x| *x * *x)
        .collect();
    for i in vec2 {
        println!("{}", i);
    }
}

fn get_even(vec: &Vec<i8>) -> Vec<i8> {
    let mut vec2 = Vec::new();
    for i in vec {
        if i % 2 == 0 {
            vec2.push(*i);
        }
    }
    return vec2;
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
