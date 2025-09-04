mod hallo;
mod variabel;
fn main() {
    hallo::sapa_dunia();
    println!("{}", variabel::variable())
}

#[test]
fn unit_test() {
    println!("Ini dari unit_test")
}
