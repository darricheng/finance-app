use serde::Deserialize;

pub mod budget;
pub mod expenses;

use budget::Budget;

#[derive(Debug, Deserialize)]
pub struct UserData {
    finances: Finances,
    budget: Budget,
}
impl UserData {
    pub fn new() -> UserData {
        UserData {
            finances: Finances::new(),
            budget: Budget::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Finances {
    pub expenses: expenses::Expenses,
    // to add income struct
}
impl Finances {
    pub fn new() -> Finances {
        Finances {
            expenses: expenses::Expenses::new(),
        }
    }
}

#[tauri::command]
pub fn print_state(state: tauri::State<crate::AppState>) {
    println!("{:?}", state.0.lock().unwrap())
}
