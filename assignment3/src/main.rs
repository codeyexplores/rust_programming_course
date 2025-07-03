/* 
Assignment 3 - Rust Programming

// define struct of UserAccount with field: name (String), and age (Option<u32>)

// define a trait called Balance, and within, function get_balance returning integer of 10

// implement trait Balance to UserAccount struct

// returning 2 values via results enum 

// create function increase_balance which takes as arguments
// - a type that implements Balance trait
// - an u32 amount parameter containing the increase amount
// within this function,
// - if increase amount is <= 10, return an OK containing the get_balance + amount
// - if increase amount is > 10, return an Err with error message "Increase must be less than 10!"
// Tip: this function should return a Result<u32, String>

fn main() {
   // create user_account, and set his age as Option::None
   
   // You want to increase the user_account's balance by 11
   // use a match, if the result of increase_balance is
   // - Ok: print "UserAccount balance increased to {}" where {} is the new balance value
   // - Err: print the error message returned
   
   // use an if...let...else statement to print the UserAccount age if it is a Option::Some
   
}

*/

// define struct of UserAccount with field: name (String), and age (Option<u32>)
struct UserAccount{
   name: String,
   age: Option<u32>,
}

impl UserAccount{
   fn new(name: String, age: Option<u32>) -> Self {
      Self {name, age}
   }
}

// define a trait called Balance, and within, function get_balance returning integer of 10
trait Balance{
   fn get_balance(&self) -> u32 {
      10
   }
}

// implement trait Balance to UserAccount struct
impl Balance for UserAccount{
   fn get_balance(&self) -> u32 {
      10
   }
}

// create function increase_balance which takes as arguments
// - a type that implements Balance trait
// - an u32 amount parameter containing the increase amount
// within this function,
// - if increase amount is <= 10, return an OK containing the get_balance + amount
// - if increase amount is > 10, return an Err with error message "Increase must be less than 10!"
// Tip: this function should return a Result<u32, String>
// returning 2 values via results enum 
fn increase_balance<T: Balance>(balance: &T, increament: u32) -> Result<u32, String> {
   let current_balance = balance.get_balance();
   match increament <= 10 {
      true => Ok(current_balance + increament),
      false => Err("Increase must be less than 10!".to_owned()),
   }
}

fn main() {
   // create user_account, and set his age as Option::None
   // You want to increase the user_account's balance by 11
   let user_account = UserAccount::new("User".to_owned(), None);
   let res = increase_balance(&user_account, 11);
   // use a match, if the result of increase_balance is
   // - Ok: print "UserAccount balance increased to {}" where {} is the new balance value
   // - Err: print the error message returned
   match res{
      Ok(new_balance) => println!("UserAccount balance increased to {}", new_balance) ,
      Err(err_msg) => println!("{}",err_msg),
   };

   // use an if...let...else statement to print the UserAccount age if it is a Option::Some
   if let Some(account_age) = &user_account.age {
      println!("UserAccount age: {:?}", account_age);
   } else {
      println!("Error: Account age not set");
   }
   println!("{}", user_account.name);
}
