mod control_flow;
mod hallo;
mod heap_stack;
mod numeric;
mod ownership;
mod tuple;
mod variabel;
fn main() {
    ownership::owner();
    heap_stack::func_a();
}

#[test]
fn test() {
    let nama = "Siam Al Sobari";
    assert_eq!(nama, "Siam Al Sobari");
    println!("Ini dari mod {}", hallo::sapa_dunia());
    println!("Ini dari mod {}", variabel::variable());
    numeric::numeric_operation();
    tuple::rust_tuple();
    control_flow::match_stat();
}

#[test]
fn string() {
    let binatang: &str = "Kucing";
    println!("Binatang ini adalah {}", binatang)
}

#[test]
fn string_type() {
    let kelas: String = String::from("X_PPLG_2");
    //kelas.push_str("XI_PPLG_4");
    let kelas_2 = kelas.clone();
    println!(
        "Saya sekarang berada di kelas {} dan clone {}",
        kelas, kelas_2
    )
}

// #[test]
// fn return_string() {
//     let nama: String = String::from("Siam Al Sobariiiiiiiiiiiiiiiiiiiiiii");
//     rs(nama);
// }
//
// fn rs(str: String) {
//     println!("{}", str)
// }
//
fn full_name(first_name: String, last_name: String) -> String {
    format!("{}{}", first_name, last_name)
}

#[test]
fn test_name() {
    let first_name: String = String::from("Siam");
    let last_name: String = String::from("Al Sobari");

    let name = full_name(first_name, last_name);
    println!("{}", name)
}
#[derive(Debug)]
struct User {
    name: String,
    umur: i32,
    email: String,
}

#[test]
fn struct_test() {
    let mut user1 = User {
        name: String::from("Siam"),
        umur: 16,
        email: String::from("Kanjud"),
    };
    user1.email = String::from("yammD@gmail.com");
    println!("{:?}", user1)
}
