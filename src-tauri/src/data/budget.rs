use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Budget {
    categories: Vec<Category>,
}
impl Budget {
    pub fn new() -> Budget {
        Budget {
            categories: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Category {
    name: String,
    amount: f64,
    aliases: Vec<String>,
}
