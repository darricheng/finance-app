use chrono::NaiveDate;
use csv;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub struct Expenses {}

#[derive(Debug, Deserialize)]
pub struct IntermediateExpenseRecord {
    date: String,
    category: String,
    amount: f64,
}

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

    pub fn from_intermediate_expense_record(
        intermediate_record: IntermediateExpenseRecord,
    ) -> ExpenseRecord {
        // date,category,amount,
        let date_str = &intermediate_record.date;

        let mut date_numbers: Vec<u32> = Vec::new();
        for num in date_str.split('/') {
            date_numbers.push(num.parse::<u32>().unwrap());
        }

        let day = date_numbers.get(0).unwrap();
        let month = date_numbers.get(1).unwrap();
        let year = date_numbers.get(2).unwrap();

        let year_i32: i32 = *year as i32;
        println!("{year}, {month}, {day}");
        let date_obj = NaiveDate::from_ymd_opt(year_i32, *month, *day).unwrap();

        ExpenseRecord::new(
            date_obj,
            intermediate_record.category,
            intermediate_record.amount,
        )
    }
}

#[tauri::command]
pub fn convert_csv_to_expense_record(
    csv_str: String,
) -> Result<Vec<ExpenseRecord>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(csv_str.as_bytes());
    let mut intermediate_records = Vec::new();
    for result in rdr.deserialize() {
        let record: IntermediateExpenseRecord = result?;
        intermediate_records.push(record);
    }

    let mut expense_records: Vec<ExpenseRecord> = Vec::new();
    for record in intermediate_records.into_iter() {
        expense_records.push(ExpenseRecord::from_intermediate_expense_record(record))
    }

    Ok(expense_records)
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
        println!(
            "[MARKER] {:?}",
            convert_csv_to_expense_record(csv_str.to_string())
        );

        assert_eq!(
            convert_csv_to_expense_record(csv_str.to_string()).unwrap(),
            vectorized_csv
        );
    }
}
