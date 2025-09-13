enum Name {
    Jokowi,
    Prabowo,
    Suharto,
}

pub fn array() {
    let names: [Name; 3] = [Name::Jokowi, Name::Prabowo, Name::Suharto];
    for name in names {
        match name {
            Name::Jokowi => println!("Ini President Jokowi"),
            Name::Prabowo => println!("Ini President Prabowo"),
            Name::Suharto => println!("Ini President Suharto"),
        }
    }
}
