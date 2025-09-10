pub fn rust_tuple() {
    let data: (i32, &str) = (3, "siam");
    println!("{:?}", data);
    println!("{}", data.0 * 5);
}
