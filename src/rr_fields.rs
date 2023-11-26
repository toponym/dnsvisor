/// Enums with values for DNS Resource Record (RR) fields
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    A = 1,
    NS = 2,
    CNAME = 5,
    MX = 15,
    TXT = 16,
    AAAA = 28,
}

impl From<u16> for Type {
    fn from(val: u16) -> Self {
        match val {
            1 => Type::A,
            2 => Type::NS,
            5 => Type::CNAME,
            15 => Type::MX,
            16 => Type::TXT,
            28 => Type::AAAA,
            _ => panic!("Integer {} does not correspond to a Type", val),
        }
    }
}

// allow non-camel case to match the DNS name for these values
#[allow(non_camel_case_types)]
pub enum Class {
    CLASS_IN = 1,
}
