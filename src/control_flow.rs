pub fn _if_stat() {
    // let num = 3;
    // if num != 0 {
    //     println!("{}", num)
    // }
    //
    let num = 15;
    if num >= 15 {
        println!("num lebih besar")
    } else {
        println!("lebih kecil")
    }

    let role = "admin";
    if role == "admin" {
        println!("admin")
    } else {
        println!("lu siapa")
    };
}

pub fn match_stat() {
    let nama = "siam";
    match nama {
        "siam" => println!("Nama dia adalah {}", { nama }),
        _ => println!("Nama tak diketahui"),
    }
}
