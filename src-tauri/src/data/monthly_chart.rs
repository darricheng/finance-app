// TODO: Add logic to generate the data needed for the monthly chart
// See the frontend for the data structure needed

use std::collections::HashMap;

use chrono::Datelike;
use serde::Serialize;
use tauri::{command, State};

use crate::AppState;

use super::{
    budget::{Budget, Category},
    expenses::Expenses,
};

// Struct for both the budget and expenses
// Data sent to frontend should be a vec of this
#[derive(Serialize)]
struct BudgetDetail {
    category: String,
    amount: f64,
}

// Struct for the monthly chart data
#[derive(Serialize)]
pub struct MonthlyData {
    budget: Vec<BudgetDetail>,
    expenses: Vec<BudgetDetail>,
}

/// Returns the data needed for the monthly chart in the shape required for chart.js
#[command]
pub fn get_monthly_chart_data(state: State<AppState>, month: u8, year: u16) -> MonthlyData {
    let user_data = state.0.lock().unwrap();
    let mut budget_state: Budget = user_data.budget.clone();
    let expenses: &Expenses = &user_data.finances.expenses;

    // TODO: use the month argument to return the correct data to the frontend
    // I think I'll also need a year argument

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

#[derive(Serialize)]
pub struct MonthYear {
    month: u8,
    year: u16,
}

#[command]
pub fn get_dates(state: State<AppState>) -> Vec<MonthYear> {
    let user_data = state.0.lock().unwrap();
    let expenses: &Expenses = &user_data.finances.expenses;

    if expenses.get_records().len() == 0 {
        return vec![];
    }

    // NOTE: I can get the year and month from the NaiveDate using the Datelike trait
    // See: https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDate.html#impl-Datelike-for-NaiveDate
    // I think I can just get the first and last months, and then generate the months in between
    let earliest_date = expenses
        .get_records()
        .iter()
        .min_by(|x, y| x.get_date().cmp(y.get_date()))
        .unwrap() // TODO: Handle the unwraps so that app doesn't crash if there is no data
        .get_date();
    let latest_date = expenses
        .get_records()
        .iter()
        .max_by(|x, y| x.get_date().cmp(y.get_date()))
        .unwrap()
        .get_date();
    let earliest_year = earliest_date.year();
    let latest_year = latest_date.year();
    let earliest_year_month = earliest_date.month();
    let latest_year_month = latest_date.month();

    let mut month_years = vec![];

    for year in earliest_year..=latest_year {
        let start_month = if year == earliest_year {
            earliest_year_month
        } else {
            1
        };
        let end_month = if year == latest_year {
            latest_year_month
        } else {
            12
        };

        for month in start_month..=end_month {
            month_years.push(MonthYear {
                month: month as u8,
                year: year as u16,
            });
        }
    }

    month_years
}
