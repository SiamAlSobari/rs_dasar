pub struct Manusia {
    pub nama: String,
}

pub trait Great {
    fn bergerak(&self, v: String);
}

impl Great for Manusia {
    fn bergerak(&self, v: String) {
        println!(
            "{} dengan nama {} sedang bergerak {}",
            "Manusia", self.nama, v
        );
    }
}
