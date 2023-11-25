/// Enums with values for DNS Resource Record (RR) fields
#[derive(Clone, Copy)]
pub enum Type {
    A = 1,
    NS = 2,
    CNAME = 5,
    MX = 15,
    TXT = 16,
}

impl From<u16> for Type {
    fn from(val: u16) -> Self {
        match val {
            1 => Type::A,
            2 => Type::NS,
            5 => Type::CNAME,
            15 => Type::MX,
            16 => Type::TXT,
            _ => panic!(),
        }
    }
}

// allow non-camel case to match the DNS name for these values
#[allow(non_camel_case_types)]
pub enum Class {
    CLASS_IN = 1,
}
