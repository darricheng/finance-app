use serde::Deserialize;
use tauri;

pub mod expenses;

#[derive(Debug, Deserialize)]
pub struct FinanceData {
    pub expenses: expenses::Expenses,
}
impl FinanceData {
    pub fn new() -> FinanceData {
        FinanceData {
            expenses: expenses::Expenses::new(),
        }
    }
}

#[tauri::command]
pub fn print_state(state: tauri::State<crate::State>) -> () {
    println!("{:?}", state.0.lock().unwrap())
}
