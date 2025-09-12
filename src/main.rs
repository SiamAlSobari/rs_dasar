mod control_flow;
mod hallo;
mod numeric;
mod tuple;
mod variabel;
fn main() {
    println!("Ini dari mod {}", hallo::sapa_dunia());
    println!("Ini dari mod {}", variabel::variable());
    numeric::numeric_operation();
    tuple::rust_tuple();
    control_flow::match_stat();
}

#[test]
fn test() {
    let nama = "Siam Al Sobari";
    assert_eq!(nama, "Siam Al Sobari");
}
