fn vector_method() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v1_it = v1.iter();
    for i in v1_it {
        println!("{}", i);
    }
    println!("{:?}", v1);
}
