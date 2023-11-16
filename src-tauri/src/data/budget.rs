use serde::{Deserialize, Serialize};
use tauri::{command, State};
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
    pub fn get_categories(&mut self) -> Vec<Category> {
        self.categories.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct Category {
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
    fn edit_category(&mut self, new_name: String, new_amount: f64, new_aliases: Vec<String>) {
        self.name = new_name;
        self.amount = new_amount;
        self.aliases = new_aliases;
    }
    pub fn get_name_and_amount(&mut self) -> (String, f64) {
        (self.name.clone(), self.amount)
    }
}

#[command]
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

#[command]
pub fn edit_budget_category(
    state: State<AppState>,
    old_name: String,
    new_name: String,
    new_amount: f64,
    new_aliases: Vec<String>,
) {
    let mut user_data = state.0.lock().unwrap();
    let some_category = user_data
        .budget
        .categories
        .iter_mut()
        .find(|x| x.name == old_name);
    if let Some(category) = some_category {
        category.edit_category(new_name, new_amount, new_aliases)
    }
}

#[command]
pub fn get_budget(state: State<AppState>) -> Budget {
    let user_data = state.0.lock().unwrap();
    user_data.budget.clone()
}
