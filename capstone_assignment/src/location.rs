/*
Defining Custom Data Types
In location.rs:
Create Country enum with simple variants of UnitedStates, 
Canada, UnitedKingdom, Germany, France, Japan, Australia, 
China, Brazil, SouthKorea, Ireland, Spain, India, Switzerland. 
Each variant does not need to encapsulate any data.
Similarly, create Continent enum with simple variants of 
NorthAmerica, Europe, Asia, Oceania, SouthAmerica.

Csv Line Reader Helpers
In location.rs:
    implement on Country
        define country_to_continent method which takes in a referenced self and return a Continent
        match on self and return an appropriate Continent variant
    implement std::str::FromStr trait on Country
        define Err type as referenced static string
        Define from_str method which takes in a referenced string and returns a Result of <Country, Country::Err>
            match on self and return an Ok encapsulating an appropriate Country variant

*/

// This module defines the Country and Continent enums
// It also provides helper methods for working with these enums.
// 
// The Country enum represents various countries,
// and each variant can then be used to determine the continent it belongs to.
 

/// Country - contains variants of countries
/// Each enum variant can have data to go along with it.
/// The Country enum implements the FromStr trait to allow for easy conversion from string literals.
#[derive(Debug)]
pub enum Country{
    UnitedStates,
    Canada, 
    UnitedKingdom, 
    Germany, 
    France, 
    Japan, 
    Australia, 
    China, 
    Brazil, 
    SouthKorea, 
    Ireland, 
    Spain, 
    India, 
    Switzerland,
}

/// The Continent enum represents the continents of the world.
#[derive(Debug, PartialEq)]
pub enum Continent{
    NorthAmerica, Europe, Asia, Oceania, SouthAmerica,
}

impl Continent {
    // Returns the string of a Continent variant
    pub fn as_string(&self) -> String {
        match self {
            Continent::NorthAmerica => "NorthAmerica".to_string(),
            Continent::Europe => "Europe".to_string(),
            Continent::Asia => "Asia".to_string(),
            Continent::Oceania => "Oceania".to_string(),
            Continent::SouthAmerica => "SouthAmerica".to_string(),
        }
    }
}

impl Country{
    // Takes a Country enum variant and returns it's respective Continent enum variant
    pub fn country_to_continent(&self) -> Continent {
        match self {
            Country::UnitedStates => Continent::NorthAmerica,
            Country::Canada => Continent::NorthAmerica, 
            Country::UnitedKingdom => Continent::Europe, 
            Country::Germany => Continent::Europe, 
            Country::France => Continent::Europe, 
            Country::Japan => Continent::Asia, 
            Country::Australia => Continent::Oceania, 
            Country::China => Continent::Asia, 
            Country::Brazil => Continent::SouthAmerica, 
            Country::SouthKorea => Continent::Asia, 
            Country::Ireland => Continent::Europe, 
            Country::Spain => Continent::Europe, 
            Country::India => Continent::Asia, 
            Country::Switzerland => Continent::Europe,
        }
    }
}

use std::str::FromStr;
impl FromStr for Country {
    type Err = &'static str;
    /// Matches input reference string slice to Country variant 
    /// Error is thrown if input is invalid
    fn from_str(s: &str) -> Result<Self, Self::Err> { // Self::Err defines the type of Err
        match s {
            "UnitedStates" => Ok(Country::UnitedStates),
            "Canada" => Ok(Country::Canada),
            "UnitedKingdom" => Ok(Country::UnitedKingdom),
            "Germany" => Ok(Country::Germany),
            "France" => Ok(Country::France),
            "Japan" => Ok(Country::Japan),
            "Australia" => Ok(Country::Australia),
            "China" => Ok(Country::China),
            "Brazil" => Ok(Country::Brazil),
            "SouthKorea" => Ok(Country::SouthKorea),
            "Ireland" => Ok(Country::Ireland),
            "Spain" => Ok(Country::Spain),
            "India" => Ok(Country::India),
            "Switzerland" => Ok(Country::Switzerland),
            _ => Err("Invalid country"),
        }   
    }
}

// Alternative implementation - without static
//
// impl std::str::FromStr for Country {
//     type Err = String;
//     /// Matches input reference string slice to Country variant 
//     /// Error is thrown if input is invalid
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "UnitedStates" => Ok(Country::UnitedStates),
//             "Canada" => Ok(Country::Canada),
//             "UnitedKingdom" => Ok(Country::UnitedKingdom),
//             "Germany" => Ok(Country::Germany),
//             "France" => Ok(Country::France),
//             "Japan" => Ok(Country::Japan),
//             "Australia" => Ok(Country::Australia),
//             "China" => Ok(Country::China),
//             "Brazil" => Ok(Country::Brazil),
//             "SouthKorea" => Ok(Country::SouthKorea),
//             "Ireland" => Ok(Country::Ireland),
//             "Spain" => Ok(Country::Spain),
//             "India" => Ok(Country::India),
//             "Switzerland" => Ok(Country::Switzerland),
//             _ => Err(format!("Invalid country : '{}'", s)),
//         }   
//     }
// }