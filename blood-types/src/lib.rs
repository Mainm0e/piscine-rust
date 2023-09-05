//todo: blood_types
/*
Instructions
In this exercise you will create a data model of blood types and an API to deal with them.

First, we'll implement the data representation of the blood types.

Take a look at the BloodType struct below. It contains two enums which enable us to describe a blood type (e.g. "A-").

Antigen: which can be one of:
A
B
AB
O
RhFactor: which can be one of:
Positive
Negative
To provide a simple way to create blood types, implement the trait FromStr for BloodType. This will allow us to use the parse method and from_str, so we can do:

   let a_neg: BloodType = "A-".parse();
Implement the following Traits:

std::cmp::Ord: to make possible to sort a vector or array of BloodType.
std::Debug: for BloodType, allowing us print a vector such as [BloodType { antigen: A, rh_factor: Positive}, BloodType{ antigen: B, rh_factor: Negative}] as "[ A+, A-]" when using format strings {:?}.
Create three methods for BloodType:

can_receive_from: which returns true if self can receive blood from other blood type.
donors: which returns all the blood types that can give blood to self.
recipients: which returns all the blood types that can receive blood from self.
Expected Functions and Structures
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
}

impl FromStr for RhFactor {
}

impl Ord for BloodType {
}

impl FromStr for BloodType {
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
    }

    pub fn donors(&self) -> Vec<Self> {
    }

    pub fn recipients(&self) -> Vec<BloodType> {
}
*/

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Copy)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

use std::borrow::BorrowMut;
use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err(()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(()),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.antigen.cmp(&other.antigen) {
            Ordering::Equal => self.rh_factor.cmp(&other.rh_factor),
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut antigen = Antigen::A;
        let mut rh_factor = RhFactor::Positive;
        let mut count = 0;
        for c in s.chars() {
            count += 1;
            // can some how check if is character start with A and next is B then set antigen to AB
            if c == 'A' && s.chars().nth(1).unwrap() == 'B' {
                antigen = Antigen::AB;
                // check if next is + or - then set rh_factor
                if s.chars().nth(2).unwrap() == '+' {
                    rh_factor = RhFactor::Positive;
                } else {
                    rh_factor = RhFactor::Negative;
                }
                break;
            }else if (c == 'A' || c == 'B' || c == 'O') && count == 1 {
            match c {
                'A' => antigen = Antigen::A,
                'B' => antigen = Antigen::B,
                'O' => antigen = Antigen::O,
                _ => return Err(()),
            }
            } else if (c == '+' || c == '-') && count == 2 {
                match c {
                    '+' => rh_factor = RhFactor::Positive,
                    '-' => rh_factor = RhFactor::Negative,
                    _ => return Err(()),
                }
            } else {
                return Err(());
            }
            }
        // check if Antigen is correct
        let check_list = vec![ Antigen::A,  Antigen::B,  Antigen::O,  Antigen::AB];
        if !check_list.contains(&antigen) {
            Err(())
        } else {
        
            print!("Antigen is not correct {:?}", antigen);
            Ok(BloodType { antigen, rh_factor })
        }
    
    }
    
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.rh_factor == RhFactor::Positive {
            write!(f, "{:?}+", self.antigen)
        } else {
            write!(f, "{:?}-", self.antigen)
        }
    }
}

impl BloodType {

    

    pub fn can_receive_from(&self, other: &Self) -> bool {
    let can_receive_map: Vec<Vec<&str>> = vec![
        vec!["A+", "AB+"],
        vec!["A+", "A-", "AB+", "AB-"],
        vec!["B+", "AB+"],
        vec!["B+", "B-", "AB+", "AB-"],
        vec!["AB+"],
        vec!["AB+", "AB-"],
        vec!["A+", "B+", "AB+", "O+"],
        vec!["A+", "A-", "B+", "B-", "AB+", "AB-", "O+", "O-"],
    ];
    let index_blood_type = vec![ "A+", "A-", "B+", "B-", "AB+", "AB-", "O+", "O-"];
    println!("other: {:?}", self);

     let mut self_str = self.convert_to_string();
     
     let mut other_str =  other.convert_to_string();
    // make other_str to & str
    let new_self_str: &str = self_str.borrow_mut();
    let new_other_str: &str = other_str.borrow_mut();

     // get index in index_blood_type
    let index_self = index_blood_type.iter().position(|&r| r == new_other_str).unwrap();
   
    if can_receive_map[index_self].contains(&new_self_str) {
        println!("{} can receive from {}:{}", new_other_str, new_self_str,new_self_str);
        return true;
    } else {
        println!("{} can't receive from {}:{}", new_other_str, new_self_str,new_self_str);
        return false;    
        
    }

    }   



    pub fn donors(&self) -> Vec<Self> {
      
    let can_doners_map: Vec<Vec<&str>> = vec![
        vec![ "A+", "A-", "O+", "O-"],
        vec![ "A-", "O-"],
        vec![ "B+", "B-", "O+", "O-"],
        vec![ "B-", "O-"],
        vec![ "A+", "A-", "B+", "B-", "AB+", "AB-", "O+", "O-"],
        vec![ "A-", "B-", "AB-", "O-"],
        vec![ "O+", "O-"],
        vec![ "O-"],
    ];
    let index_blood_type = vec![ "A+", "A-", "B+", "B-", "AB+", "AB-", "O+", "O-"];


     let mut self_str = self.convert_to_string();
     let new_self_str: &str = self_str.borrow_mut();
     let index_self = index_blood_type.iter().position(|&r| r == new_self_str).unwrap();
     let can_doners_map = can_doners_map[index_self].clone();
     let mut donors = Vec::new();
        for blood_type in can_doners_map {
            let blood_type: BloodType = blood_type.parse().unwrap();
            donors.push(blood_type);
        }
        donors

    }
        

    pub fn recipients(&self) -> Vec<BloodType> {
        let mut compatible_recipients = Vec::new();

        for antigen in &[Antigen::A, Antigen::B, Antigen::AB, Antigen::O] {
            for rh_factor in &[RhFactor::Positive, RhFactor::Negative] {
                let blood_type = BloodType {
                    antigen: *antigen,
                    rh_factor: *rh_factor,
                };

                if blood_type.can_receive_from(self) {
                    compatible_recipients.push(blood_type);
                }
            }
        }

        compatible_recipients
    }

    // just convert blood type to string
    fn convert_to_string(&self) -> String {
        let antigen = match self.antigen {
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::AB => "AB",
            Antigen::O => "O",
        };
    
        let rh_factor = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
    
        let blood_type = format!("{}{}", antigen, rh_factor);
        
        blood_type
    }
    
}
