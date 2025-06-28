// Assignment 1 - Rust Programming 

// Define a User struct which contains 2 fields: 
// name (string e.g "John")
// balance (tuple e.g (100.00, "SGD"))
struct User {
    name: String,
    balance: (f64, String), // (amount, currency)
}

// Define a User method (using impl) called print_user_detail, which simply prints the name, balance and currency of the user.
impl User {
    fn new(name: String, balance: (f64, String)) -> Self {
        Self { name, balance }
    }

    fn print_user_detail(&self) {
        println!("Name: {}", self.name);
        println!("Balance: {} {}", self.balance.0, self.balance.1);
    }
}

// Define an accrue_interest function, which takes in a user and interest percentage as 2 separate parameters. 
// Within the function, increase the users' balance by the interest percentage, and print out the user details by calling the print_user_detail method.
fn accrue_interest(user: &mut User, interest_rate: f64) {
    user.balance.0 += user.balance.0 * interest_rate;
    user.print_user_detail();
}

// In the main function, create a user variable of type User, populating the field values of name, and balance and currency. Then, call the accrue_interest function. 
// Bonus: After the call to accrue_interest, call it multiple times so that the user may benefit from compounding interest. 
fn main() {
    let mut number_of_years = 5; // Number of years to accrue interest
    let interest_rate = 0.03; // 3% interest rate
    let mut user = User::new(
        String::from("John"),
        (100.00, String::from("SGD")),
    );
    println!("Initial account details:\n Name:{} \n Balance: {} {}", user.name, user.balance.0, user.balance.1);
    let mut current_year = 0;
    while number_of_years > 0 {
        println!("\n At end of Year {}:", current_year + 1);
        current_year += 1;
        accrue_interest(&mut user, interest_rate);
        number_of_years -= 1;
    }
}

/*
Assignment
Define a User struct which contains 2 fields: 
name (string e.g "John")
balance (tuple e.g (100.00, "SGD"))
Define a User method (using impl) called print_user_detail, which simply prints the name, balance and currency of the user.
Define an accrue_interest function, which takes in a user and interest percentage as 2 separate parameters. Within the function, increase the users' balance by the interest percentage, and print out the user details by calling the print_user_detail method.
In the main function, create a user variable of type User, populating the field values of name, and balance and currency. Then, call the accrue_interest function. 
Bonus: After the call to accrue_interest, call it multiple times so that the user may benefit from compounding interest. 
*/
