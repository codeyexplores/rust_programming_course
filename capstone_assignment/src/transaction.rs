/*
Defining Custom Data Types
In transaction.rs:
    use crate::location::{Country, Continent};
    Define Transaction struct with fields 
        transaction_id: unsigned 32bit integer
        client_id: unsigned 32bit integer
        asset_name: owned string
        country: Country enum
        continent: Continent enum
        amount: float 64bit number
        days_under_management: signed 64bit integer

Defining Csv Line Reader Function 
In transaction.rs:
    Implement a public function from_csv_line on Transaction type. The function should take in an input, 
    line, of type string slice (1 row of data) and output a Result of type 
    Transaction if Ok, String if Err
        -create a variable fields by calling the split method using line with the delimiter ',' 
        followed by the collect method to transform it to a Vec of string slices
        -check if the length of fields is equals to 7, if it is not, return an Err with an 
        appropriate message in String
        -create a transaction_id variable by using the first item of fields to call the 
        parse::<u32> method followed by an unwrap method 
        -create a client_id variable by using the second item of fields to call the 
        parse::<u32> method followed by an unwrap method 
        -create a asset_name variable by using the third item of fields to call the to_uppercase method
        -create a transaction_start_date variable by passing the fourth item of fields 
        into NaiveDate::parse_from_str function followed by an unwrap method 
        -create a transaction_end_date variable by passing the fifth item of fields 
        into NaiveDate::parse_from_str function followed by an unwrap method 
        -create a country variable by using the sixth item of fields to call the 
        parse::<Country> method followed by the ? operator
        -create a amount variable by using the seventh item of fields to call the 
        parse::<f64> method followed by an unwrap method
        -create a days_under_management variable by taking the result of 
        subtracting transaction_start_date with transaction_end_date followed by calling num_days method
        -create a continent variable by passing a reference of country into the 
        country_to_continent function
        -create a transaction variable by instantiating an instance of Transaction 
        with the variables we have just created
        -return an Ok which encapsulates the transaction variable we just created
*/

// This module defines the Transaction struct and its methods for processing CSV lines.

// use crate goes to the root directory
pub use crate::location::{Country, Continent};
pub use crate::transaction;
pub use chrono::NaiveDate;

/// Transaction struct represents a financial transaction with various attributes
/// It includes fields such as transaction ID, client ID, asset name, country, continent,
/// amount, and the number of days the asset was under management
#[derive(Debug)]
pub struct Transaction{
        pub transaction_id: u32,
        pub client_id: u32,
        pub asset_name: String,
        pub country: Country,
        pub continent: Continent,
        pub amount: f64,
        pub days_under_management: i64
}

impl Transaction{
    /// Checks and formats transactions that are valid for storing
    /// Creates a transaction variable/instance if csv line is valid with 7 fields
    /// Throws an error if number of fields is invalid
    pub fn from_csv_line(line: &str) -> Result<Transaction, String> {
        let fields: Vec<&str> = line.split(',').collect();

        if fields.len() != 7 {
            return Err("Invalid CSV line".to_owned());
        }

        let transaction_id = fields[0].parse::<u32>().unwrap(); 
            // can use match || .unwrap_or_else to handle .parse return
        let client_id = fields[1].parse::<u32>().unwrap();
        let asset_name = fields[2].to_uppercase();
        let transaction_start_date = NaiveDate::parse_from_str(fields[3], "%Y-%m-%d").unwrap();
        let transaction_end_date = NaiveDate::parse_from_str(fields[4], "%Y-%m-%d").unwrap();
        let country = fields[5].parse::<Country>()?;
            // parse here actually calls the FromStr which we have a variant 
            // alternatively: let country: Country = fields[5].parse()?;
            // similar to doing: let country: Country::from_str(fields[5]);
        let amount = fields[6].parse::<f64>().unwrap();
        let days_under_management = (transaction_end_date - transaction_start_date).num_days();
        let continent = country.country_to_continent();

        let transaction = Transaction {
            transaction_id,
            client_id,
            asset_name,
            country,
            continent,
            amount,
            days_under_management,
        };

        Ok(transaction)
    }
}