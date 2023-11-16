// TODO: Add logic to generate the data needed for the monthly chart
// See the frontend for the data structure needed

use std::collections::HashMap;

use serde::Serialize;
use tauri::{command, State};

use crate::AppState;

use super::budget::{Budget, Category};

// Struct for both the budget and expenses
// Data sent to frontend should be a vec of this
#[derive(Serialize)]
struct BudgetDetail {
    category: String,
    amount: f64,
}
#[derive(Serialize)]
struct BudgetData(Vec<BudgetDetail>);

// Struct for the monthly chart data
#[derive(Serialize)]
pub struct MonthlyData {
    budget: Vec<BudgetDetail>,
    expenses: Vec<BudgetDetail>,
}

#[command]
pub fn get_monthly_chart_data(state: State<AppState>) -> MonthlyData {
    let user_data = state.0.lock().unwrap();
    let mut budget_state: Budget = user_data.budget.clone();
    let expenses = &user_data.finances.expenses;

    let mut categories: HashMap<String, f64> = HashMap::new();
    budget_state
        .get_categories()
        .iter_mut()
        .for_each(|category: &mut Category| {
            let (name, amount) = category.get_name_and_amount();
            categories.insert(name, amount);
        });
    let budget = categories
        .iter()
        .map(|(name, amount)| BudgetDetail {
            category: name.clone(),
            amount: *amount,
        })
        .collect::<Vec<BudgetDetail>>();

    MonthlyData {
        budget,
        expenses: vec![],
    }
}
