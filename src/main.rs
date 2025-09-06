mod hallo;
mod numeric;
mod variabel;

fn main() {
    println!("Ini dari mod {}", hallo::sapa_dunia());
    println!("Ini dari mod {}", variabel::variable());
    numeric::numeric_operation();
}

#[test]
fn test() {
    let nama = "Siam Al Sobari";
    assert_eq!(nama, "Siam Al Sobari");
}
