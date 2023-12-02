/// Enums with values for DNS Resource Record (RR) fields
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Type {
    A = 1,
    NS = 2,
    CNAME = 5,
    MX = 15,
    TXT = 16,
    AAAA = 28,
}

impl TryFrom<u16> for Type {
    type Error = String;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            1 => Ok(Type::A),
            2 => Ok(Type::NS),
            5 => Ok(Type::CNAME),
            15 => Ok(Type::MX),
            16 => Ok(Type::TXT),
            28 => Ok(Type::AAAA),
            _ => Err(format!("Integer {} does not correspond to a Type", val)),
        }
    }
}

// allow non-camel case to match the DNS name for these values
#[allow(non_camel_case_types)]
pub enum Class {
    CLASS_IN = 1,
}
