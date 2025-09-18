use crate::method::Great;

mod arr;
mod control_flow;
mod hallo;
mod heap_stack;
mod method;
mod numeric;
mod ownership;
mod tuple;
mod variabel;
fn main() {
    // ownership::owner();
    // heap_stack::func_a();
    // arr::array();
    // let orang = method::Person {
    //     nama: String::from("YammD"),
    // };
    // orang.cetak("Kanjud");
    let man = method::Manusia {
        nama: String::from("Yd"),
    };
    man.bergerak(String::from("lari ke sekolah"));
    let arr = [1, 2, 3, 4, 5];
    let db: Vec<i32> = arr.iter().map(|x| x * 2).collect();
    for v in db {
        println!("{:?}", v)
    }
}
//
// #[test]
// fn test() {
//     let nama = "Siam Al Sobari";
//     assert_eq!(nama, "Siam Al Sobari");
//     println!("Ini dari mod {}", hallo::sapa_dunia());
//     println!("Ini dari mod {}", variabel::variable());
//     numeric::numeric_operation();
//     tuple::rust_tuple();
//     control_flow::match_stat();
// }
//
// #[test]
// fn string() {
//     let binatang: &str = "Kucing";
//     println!("Binatang ini adalah {}", binatang)
// }
//
// #[test]
// fn string_type() {
//     let kelas: String = String::from("X_PPLG_2");
//     //kelas.push_str("XI_PPLG_4");
//     let kelas_2 = kelas.clone();
//     println!(
//         "Saya sekarang berada di kelas {} dan clone {}",
//         kelas, kelas_2
//     )
// }
//
// // #[test]
// // fn return_string() {
// //     let nama: String = String::from("Siam Al Sobariiiiiiiiiiiiiiiiiiiiiii");
// //     rs(nama);
// // }
// //
// // fn rs(str: String) {
// //     println!("{}", str)
// // }
// //
// fn full_name(first_name: String, last_name: String) -> String {
//     format!("{}{}", first_name, last_name)
// }
//
// #[test]
// fn test_name() {
//     let first_name: String = String::from("Siam");
//     let last_name: String = String::from("Al Sobari");
//
//     let name = full_name(first_name, last_name);
//     println!("{}", name)
// }
// #[derive(Debug)]
// struct User {
//     name: String,
//     umur: i32,
//     email: String,
// }
//
// #[test]
// fn struct_test() {
//     let mut user1 = User {
//         name: String::from("Siam Pemakan Kecoa"),
//         umur: 16,
//         email: String::from("Kanjud"),
//     };
//     user1.email = String::from("yammD@gmail.com");
//     println!("{:?}", user1)
// }
//
// fn bagi(a: i32, b: i32) -> Result<i32, String> {
//     match b {
//         0 => Err(String::from("Ga bisa di bagi")),
//         _ => Ok(a / b),
//     }
// }
//
// #[test]
// fn test_bagi() {
//     match bagi(10, 0) {
//         Ok(hasil) => println!("Hasil {hasil}"),
//         Err(err) => println!("{err} "),
//     }
// }
//
// #[test]
// fn test_refrence() {
//     let jenis_kelamin: String = String::from("Laki-Laki");
//     let jk = &jenis_kelamin;
//     println!("{jenis_kelamin} dan {jk}")
// }
//
// fn cetak(karakter: &mut String) {
//     karakter.push_str("Thomas andree");
//     println!("{karakter}")
// }
//
// #[test]
// fn test_refrence2() {
//     let mut karak: String = String::from("Sung Jin Woo");
//     cetak(&mut karak);
//     println!("{}", karak)
// }
