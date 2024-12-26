use serde::{Serialize, Deserialize};
use lettre::Address;


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EmailAddress {
    New(Address),
    Verified(Address),
}


impl From<EmailAddress> for Address {
    fn from(email: EmailAddress) -> Self {
        match email {
            EmailAddress::New(address) => address,
            EmailAddress::Verified(address) => address
        }
    }
}