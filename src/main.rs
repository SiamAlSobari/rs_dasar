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
    let mut binatang: &str = "Kucing";
    binatang = "Monyet";
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
