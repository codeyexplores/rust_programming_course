/*
Assignment 2 - Rust Programming

Vectors, Ownership

You will create Buyers with different Payment Types, and assign them to a Buyer Group. 
A Buyer must keep buying from a Seller until his balance is insufficient

Define an Enum PaymentType with variants:
    DigitalToken
    Cash
Define a Seller struct which contains 3 fields:
    payment_type (PaymentType)
    price (f32)
    balance (f32)
Define a Buyer struct which contains 3 fields:
    name (String)
    payment_type (PaymentType)
    balance (f32)
Define a BuyerGroup struct that contains a vector of members (a vector of Buyer struct).

Implement method add_member on BuyerGroup which adds a Buyer into members vector
Implement method find_buyer on BuyerGroup that finds returns index of Buyer with matching payment_type, 
otherwise return -1
Implement method buy on BuyerGroup which accepts a buyer index, reference to a seller, 
and keeps transferring value of seller.price from buyer to seller, until the buyer's balance is insufficient.
In the main function:
    Create 2 buyers with names John, Sally, with payment_type of DigitalToken, Cash, 
    and balance of 100.00 and 100.00 respectively
    Create an empty BuyerGroup
    Add 2 buyers (John and Sally) into buyer_group sequentially
    Create 1 seller with payment_type of Cash, price of 10, balance of 0
    Call find_buyer method on the buyer group to get index of Sally
    Call buy method on the buyer group passing the index of Sally, and the seller

*/

//Define an Enum PaymentType with variants:
#[derive(PartialEq)]
enum PaymentType {
    DigitalToken,
    Cash,
}

// Define a Seller struct which contains 3 fields:
struct Seller {
    payment_type: PaymentType,
    price: f32,
    balance: f32,
}

// Define a Buyer struct which contains 3 fields:
struct Buyer {
    name: String,
    payment_type: PaymentType,
    balance: f32,
}

// Define a BuyerGroup struct that contains a vector of members (a vector of Buyer struct).
struct BuyerGroup {
    members: Vec<Buyer>,
}

// Implement method add_member on BuyerGroup which adds a Buyer into members vector
impl BuyerGroup {
    fn new() -> Self {
        Self { members: Vec::new() }
    }
    fn add_member(&mut self, buyer: Buyer) {
        self.members.push(buyer);
    }
    //Implement method find_buyer on BuyerGroup that finds returns index of Buyer with matching payment_type, 
    //otherwise return -1
    fn find_buyer(&self, payment_type: &PaymentType) -> i32 {
        let mut index = 0;
        for buyer in &self.members {
            if buyer.payment_type == *payment_type {
                return index;
            }
            index += 1;
        }

        -1
    }
    // Implement method buy on BuyerGroup which accepts a buyer index, reference to a seller, 
    // and keeps transferring value of seller.price from buyer to seller, 
    // until the buyer's balance is insufficient.
    fn buy(&mut self, buyer_index: i32, seller: &mut Seller) {
        if buyer_index < 0 || (buyer_index as usize) >= self.members.len() {
            println!("Invalid buyer index");
            return;
        }
        let buyer = &mut self.members[buyer_index as usize];
        while buyer.balance >= seller.price {
            buyer.balance -= seller.price;
            seller.balance += seller.price;
            println!("Value of {} transferred from {} to seller, remaining balance: {}", seller.price, buyer.name, buyer.balance);
        }
        println!("{} has insufficient balance to purchase, remaining balance: {}", buyer.name, buyer.balance);
    }
}

fn main() {
    // Create 2 buyers with names John, Sally, with payment_type of DigitalToken, Cash, 
    // and balance of 100.00 and 100.00 respectively
    let mut buyer1 = Buyer{
        name: String::from("John"),
        payment_type: PaymentType::DigitalToken,
        balance: 100.00,
    };
    
    let mut buyer2 = Buyer{
        name: String::from("Sally"),
        payment_type: PaymentType::Cash,
        balance: 100.00,
    };
    
    // Create an empty BuyerGroup
    let mut buyer_group = BuyerGroup::new();
    
    // Add 2 buyers (John and Sally) into buyer_group sequentially
    buyer_group.add_member(buyer1);
    buyer_group.add_member(buyer2);

    // Create 1 seller with payment_type of Cash, price of 10, balance of 0
    let mut seller1 = Seller{
        balance: 0.0,
        payment_type: PaymentType::Cash,
        price: 10.0,
    };
    
    // Call find_buyer method on the buyer group to get index of Sally
    let buyer_index = buyer_group.find_buyer(&PaymentType::Cash);
    
    // Call buy method on the buyer group passing the index of Sally, and the seller
    buyer_group.buy(buyer_index, &mut seller1);
}
