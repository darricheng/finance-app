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
    fn add_category(&mut self, category: Category) {
        self.categories.push(category);
    }
}

#[derive(Debug, Deserialize)]
struct Category {
    name: String,
    amount: f64,
    aliases: Vec<String>,
}
impl Category {
    pub fn new(name: String, amount: f64, aliases: Vec<String>) -> Category {
        Category {
            name,
            amount,
            aliases,
        }
    }
}

#[tauri::command]
pub fn add_new_budget_category(
    state: tauri::State<crate::AppState>,
    name: String,
    amount: f64,
    aliases: Vec<String>,
) {
    let category = Category::new(name, amount, aliases);

    let mut user_data = state.0.lock().unwrap();
    user_data.budget.add_category(category);
}
