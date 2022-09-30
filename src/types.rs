#[derive(Debug, PartialEq, Eq)]
pub enum BoardColor {
    White = 0,
    Black = 1,
}

pub struct Player {
    pub name: String,
    pub title: Option<String>,
    pub rate: u32
}

impl Player {
    pub fn new(name: &str, rate: u32, title: &str) -> Self {
        Self {
            name: name.to_owned(),
            title: if title.is_empty() { None } else { Some(title.to_owned()) },
            rate
        }
    }
}


