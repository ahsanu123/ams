#[derive(Default)]
pub struct User {}

pub struct Shop {
    name: String,
}

impl Default for Shop {
    fn default() -> Self {
        Shop {
            name: String::from("fish shop"),
        }
    }
}
