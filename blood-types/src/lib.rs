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

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

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

        for c in s.chars() {
            match c {
                'A' | 'B' | 'O' => antigen = Antigen::from_str(&c.to_string()).unwrap(),
                '+' | '-' => rh_factor = RhFactor::from_str(&c.to_string()).unwrap(),
                _ => return Err(()),
            }
        }

        Ok(BloodType { antigen, rh_factor })
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

        /*

        write!(f, "{:?}{:?}", self.antigen, self.rh_factor) */
    }
}

impl BloodType {

    pub fn can_receive_from(&self, other: &Self) -> bool {
        // Check if Rh factors are the same
        if self.rh_factor != other.rh_factor {
            return false;
        }

        // Check antigen compatibility based on the Rh factor
        match self.rh_factor {
            RhFactor::Positive => {
                match self.antigen {
                    Antigen::A => match other.antigen {
                        Antigen::A | Antigen::O => true,
                        _ => false,
                    },
                    Antigen::B => match other.antigen {
                        Antigen::B | Antigen::O => true,
                        _ => false,
                    },
                    Antigen::AB => true, // AB+ can receive from any blood type
                    Antigen::O => true,  // O+ can receive from any blood type
                }
            }
            RhFactor::Negative => {
                // Rh-negative can only receive from Rh-negative
                self.rh_factor == other.rh_factor
            }
        }
    }

    pub fn donors(&self) -> Vec<Self> {
        let mut donors = vec![];
        
            match self.rh_factor {
                RhFactor::Positive => {
                    match self.antigen {
                        Antigen::A => {
                            donors.push(BloodType {
                                antigen: Antigen::A,
                                rh_factor: RhFactor::Positive,
                            });
                            donors.push(BloodType {
                                antigen: Antigen::A,
                                rh_factor: RhFactor::Negative,
                            });
                            donors.push(BloodType {
                                antigen: Antigen::O,
                                rh_factor: RhFactor::Positive,
                            });
                            donors.push(BloodType {
                                antigen: Antigen::O,
                                rh_factor: RhFactor::Negative,
                            });
                        }
                        Antigen::B => {
                            donors.push(BloodType {
                                antigen: Antigen::B,
                                rh_factor: RhFactor::Positive,
                            });
                            donors.push(BloodType {
                                antigen: Antigen::B,
                                rh_factor: RhFactor::Negative,
                            });
                            donors.push(BloodType {
                                antigen: Antigen::O,
                                rh_factor: RhFactor::Positive,
                            });
                            donors.push(BloodType {
                                antigen: Antigen::O,
                                rh_factor: RhFactor::Negative,
                            });
                        }
                        Antigen::AB => {
                            donors.push(BloodType {
                                antigen: Antigen::A,
                                rh_factor: RhFactor::Positive,
                            });
                            donors.push(BloodType {
                                antigen: Antigen::A,
                                rh_factor: RhFactor::Negative,
                            });
                            donors.push(BloodType {
                                antigen: Antigen::B,
                                rh_factor: RhFactor::Positive,
                            });
                            donors.push(BloodType {
                                antigen: Antigen::B,
                                rh_factor: RhFactor::Negative,
                            });
                            donors.push(BloodType {
                                antigen: Antigen::AB,
                                rh_factor: RhFactor::Positive,
                            });
                            donors.push(BloodType {
                                antigen: Antigen::AB,
                                rh_factor: RhFactor::Negative,
                            });
                            donors.push(BloodType {
                                antigen: Antigen::O,
                                rh_factor: RhFactor::Positive,
                            });
                            donors.push(BloodType {
                                antigen: Antigen::O,
                                rh_factor: RhFactor::Negative,
                            });
                        }
                        Antigen::O => {
                            donors.push(BloodType {
                                antigen: Antigen::O,
                                rh_factor: RhFactor::Positive,
                            });
                            donors.push(BloodType {
                                antigen: Antigen::O,
                                rh_factor: RhFactor::Negative,
                            });
                        }
                    }
                }
                RhFactor::Negative => {
                    donors.push(BloodType {
                        antigen: self.antigen,
                        rh_factor: RhFactor::Negative,
                    });
                }
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
}
