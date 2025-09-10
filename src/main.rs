mod hallo;
mod numeric;
mod tuple;
mod variabel;
fn main() {
    println!("Ini dari mod {}", hallo::sapa_dunia());
    println!("Ini dari mod {}", variabel::variable());
    numeric::numeric_operation();
    tuple::rust_tuple();
}

#[test]
fn test() {
    let nama = "Siam Al Sobari";
    assert_eq!(nama, "Siam Al Sobari");
}
