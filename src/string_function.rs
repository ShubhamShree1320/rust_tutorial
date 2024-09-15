pub fn string_function(na: &str) {
    let length = get_str_len(na);
    println!("{}", length);
}
fn get_str_len(n: &str) -> i32 {
    n.chars().count().try_into().unwrap()
}
