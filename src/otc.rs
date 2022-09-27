use std::fs;

pub struct Source {
    pub name: String,
    pub contents: String,
}

impl Source {
    const EXTENSION: &'static str = "tr";

    pub fn of(name: String) -> Self {
        let path = format!("{}.{}", &name, Self::EXTENSION);
        let contents = fs::read_to_string(path).expect("Could not open the file!");
        Self { name, contents }
    }
}
