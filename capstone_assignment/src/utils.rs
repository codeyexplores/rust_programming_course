// HashMap and sorting related functions
// 
// Utilize HashMap to keep track of the total invested amount per continent 
// and print the result out for each continent
//     Hint: You would need to convert the continent to String to store as keys
// Create a function that takes in a reference slice of transactions and 
// a reference of Continent, and filters rows by the Continent. 
// Print only transactions with European companies
//     Hint: You would need to utilise iterators, and filter function


use std::collections::HashMap;
pub use crate::location::Continent;
pub use crate::transaction::Transaction;

/// Creates a HashMap<String, f64> using transactions. 
/// Key = Continent as String, Value = Amount invested
/// Matches by the continent and accumulates the total amount for the continent
pub fn print_total_invested_amount_per_continent(transactions: &mut Vec<Transaction>) -> () {
    let mut continent_amount_map:HashMap<String, f64> = HashMap::new();
    continent_amount_map.insert(Continent::NorthAmerica.as_string(), 0.0);
    continent_amount_map.insert(Continent::Europe.as_string(), 0.0); 
    continent_amount_map.insert(Continent::Asia.as_string(), 0.0);
    continent_amount_map.insert(Continent::Oceania.as_string(), 0.0);
    continent_amount_map.insert(Continent::SouthAmerica.as_string(), 0.0);
    for tx in transactions {
        let key = tx.continent.as_string();
        match continent_amount_map.get_mut(&key) {
            Some(amount) => {
                *amount += tx.amount; // Update in place
            }
            None => {
                // Optional: handle unknown continent
                println!("Continent not found in map: {}", key);
            }
        }
    }
    println!("{:?}\n", continent_amount_map);
}

/// Filters and prints rows by the Continent
/// Takes a Vector of transactions and specified continent and prints list of
/// transactions by the continent
// Challenge: go to all europe countries then add them
pub fn print_transaction_by_continent(transactions: &mut Vec<Transaction>, continent: &Continent) {
    let mut tx_map_by_continent:HashMap<u32, &Transaction> = HashMap::new();
    let mut total = 0.0;

    println!("List of transactions by {}", continent.as_string());
    for tx in transactions {
        if &tx.continent == continent {
            tx_map_by_continent.insert(tx.transaction_id, tx);
            println!("{:?}", tx);

            // Accumulated amount
            total += tx.amount;
        }
    }
    println!("Total investment in transactions {}\n", total);
    // println!("{:?}", tx_map_by_continent);
}

/// Filters and prints rows by the Continent
/// For capstone requirement using iterators and filters => Returns a HashMap<u32, &Transaction>
pub fn alt_print_transaction_by_continent<'a>(transactions: &'a Vec<Transaction>, continent: &Continent) -> HashMap<u32, &'a Transaction> {
    let mut tx_map_by_continent:HashMap<u32, &Transaction> = HashMap::new();
    let mut total:f64 = 0.0;

    println!("Alternative Generated List of transactions by {}", continent.as_string());

    // Filter transactions by continent
    let filtered: Vec<&Transaction> = transactions
        .iter()
        .filter(|tx| &tx.continent == continent)
        .collect();
    
    tx_map_by_continent = filtered
        .iter()
        .map(|tx| (tx.transaction_id, *tx))
        .collect();
    
    // Print transaction
    for tx in &filtered {
        println!("{:?}", tx);
    }

    total = filtered.iter().map(|tx| tx.amount).sum();

    println!("Total investment in transactions {}\n", total);
    tx_map_by_continent
}


/* //[Ignore Below] - side quest
// 
// Create HashMap of transactions tied to id
// allow parsing of values in transactions 

// Non-working code, stored as potential exploration to create HashMap from a HashMap


// Creates a hashmap of transactions, using their id as the key value
// This process consumes the original vector of Transaction!
// Similar id numbers are not handled and may overwrite data unknowingly!
pub fn create_transactions_hashmap(transactions: &mut Vec<Transaction>) -> HashMap<u32, Transaction> {
    let mut tx_map:HashMap<u32, Transaction> = HashMap::new();
    for row in transactions.drain(..) {
        tx_map.insert(row.transaction_id, row);
    }
    tx_map
}

// Creates a second hashmap of transactions that have matching input continent
pub fn create_continent_map(country: &str, tx_map: HashMap<u32, Transaction>)-> HashMap<String, Continent> {
    type new_map = HashMap<String, Continent>;
    for tx in tx_map.keys() {
        match tx.get() {
            Some(country) => new_map.insert(tx),
            None => tx.key()
        }
    }
    new_map
}

// Sample transaction row
// Transaction { transaction_id: 4, client_id: 103, asset_name: "ALIBABA", country: China, continent: Asia, amount: 4000.0, days_under_management: 16 }
// Transaction { transaction_id: 6, client_id: 105, asset_name: "VOLKSWAGEN", country: Germany, continent: Europe, amount: 3500.0, days_under_management: 13 }
// Transaction { transaction_id: 7, client_id: 106, asset_name: "TENCENT", country: China, continent: Asia, amount: 2000.0, days_under_management: 15 }
// Transaction { transaction_id: 9, client_id: 108, asset_name: "ADIDAS", country: Germany, continent: Europe, amount: 1200.0, days_under_management: 10 }

// Syntax reference for HashMap
// use std::collections::HashMap;

// fn main() {
//    let mut users = HashMap::new(); // {}
//    users.insert("Jack", 21); // {"Jack": 21}
//    users.insert("Jill", 18); // {"Jack": 21, "Jill": 18}
//    users.insert("Adam", 15); // {"Jack": 21, "Jill": 18, "Adam": 15}
//    users.remove("Jill");	   // {"Jack: 21, Adam": 15}
//    users.insert("Jack", 50); // update existing entry
  
//    match users.get("Adam") { //.get() returns an Option type!!
//        Some(age) => println!("Adam's age: {:?}", age), // prints 15
//        None => println!("Not found"),
//    } 
//    match users.get("Jill") { //.get() returns an Option type!!
//        Some(age) => println!("Jill's age: {:?}", age),  // does this print?
//        None => println!("Not found"),                              // does this print?
//    } 
//    match users.get("Jack") { //.get() returns an Option type!!
//        Some(age) => println!("Jack's age: {:?}", age),  // what does this print?
//        None => println!("Not found"),
//    } 
//    for person in users.keys() { // .keys() returns keys only
//        println!("person = {}", person); // prints Jack, Adam
//    }
// }
*/