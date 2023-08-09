use chrono::NaiveDate;
use csv;
use serde::{Deserialize, Serialize};
use tauri;

use crate::errors::Error;

#[derive(Debug, Deserialize)]
pub struct Expenses {
    records: Vec<ExpenseRecord>,
}
impl Expenses {
    pub fn new() -> Expenses {
        Expenses {
            records: Vec::new(),
        }
    }
}

/// Intermediate struct used to deserialize CSV data into ExpenseRecord structs
#[derive(Debug, Deserialize)]
pub struct IntermediateExpenseRecord {
    date: String,
    category: String,
    amount: f64,
}

/// Main struct used to represent the individual expense data
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExpenseRecord {
    date: NaiveDate,
    category: String,
    amount: f64,
}
impl ExpenseRecord {
    pub fn new(date: NaiveDate, category: String, amount: f64) -> ExpenseRecord {
        ExpenseRecord {
            date,
            category,
            amount,
        }
    }
    /// Converts an IntermediateExpenseRecord into an ExpenseRecord
    pub fn from_intermediate_expense_record(
        intermediate_record: IntermediateExpenseRecord,
    ) -> Result<ExpenseRecord, Error> {
        // date,category,amount,
        let date_str = &intermediate_record.date;

        let mut date_numbers: Vec<&str> = Vec::new();
        for num in date_str.split('/') {
            date_numbers.push(num);
        }

        let (day_str, month_str, year_str) = match (
            date_numbers.first(),
            date_numbers.get(1),
            date_numbers.get(2),
        ) {
            (Some(day), Some(month), Some(year)) => (day, month, year),
            _ => {
                return Err(Error::Date {
                    reason: "Invalid date format".to_string(),
                })
            }
        };

        let (day, month, year) = match (
            day_str.parse::<u32>(),
            month_str.parse::<u32>(),
            year_str.parse::<i32>(),
        ) {
            (Ok(day), Ok(month), Ok(year)) => (day, month, year),
            _ => {
                return Err(Error::Date {
                    reason: "Invalid date value(s)".to_string(),
                })
            }
        };

        let date_obj = match NaiveDate::from_ymd_opt(year, month, day) {
            Some(date) => date,
            None => {
                return Err(Error::Date {
                    reason: "Invalid date value(s)".to_string(),
                })
            }
        };

        Ok(ExpenseRecord::new(
            date_obj,
            intermediate_record.category,
            intermediate_record.amount,
        ))
    }
}

pub fn convert_csv_to_expense_record(csv_str: String) -> Result<Vec<ExpenseRecord>, Error> {
    let mut rdr = csv::ReaderBuilder::new()
        // input data just as is without the headers, weird to add headers when inputting data
        // FUTURE: data validation should be done in the frontend
        .has_headers(false)
        .from_reader(csv_str.as_bytes());
    let mut expense_records = Vec::new();
    for result in rdr.deserialize() {
        let record: IntermediateExpenseRecord = result?;
        expense_records.push(ExpenseRecord::from_intermediate_expense_record(record)?);
    }

    Ok(expense_records)
}

#[tauri::command]
pub fn add_expenses(state: tauri::State<crate::State>, data: String) -> Result<(), Error> {
    let expenses = convert_csv_to_expense_record(data)?;

    let mut expenses_data = state.0.lock().unwrap();

    for record in expenses.into_iter() {
        expenses_data.finances.expenses.records.push(record);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_to_vec() {
        let csv_str = "date,category,amount,
28/03/2023,Food,-1,Lunch
12/09/2023,Transport,2,";
        let vectorized_csv = vec![
            ExpenseRecord {
                date: NaiveDate::from_ymd_opt(2023, 3, 28).unwrap(),
                category: "Food".to_string(),
                amount: -1.0,
            },
            ExpenseRecord {
                date: NaiveDate::from_ymd_opt(2023, 9, 12).unwrap(),
                category: "Transport".to_string(),
                amount: 2.0,
            },
        ];

        assert_eq!(
            convert_csv_to_expense_record(csv_str.to_string()).unwrap(),
            vectorized_csv
        );
    }
}
