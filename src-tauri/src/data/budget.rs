use serde::{Deserialize, Serialize};
use tauri::State;
use ts_rs::TS;

use crate::AppState;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
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

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
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
    state: State<AppState>,
    name: String,
    amount: f64,
    aliases: Vec<String>,
) {
    let category = Category::new(name, amount, aliases);

    let mut user_data = state.0.lock().unwrap();
    user_data.budget.add_category(category);
}
