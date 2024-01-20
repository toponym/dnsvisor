use crate::error::DnsError;
/// Enums with values for DNS Resource Record (RR) fields
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Type {
    A = 1,
    NS = 2,
    CNAME = 5,
    MX = 15,
    TXT = 16,
    AAAA = 28,
}

impl TryFrom<u16> for Type {
    type Error = DnsError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            1 => Ok(Type::A),
            2 => Ok(Type::NS),
            5 => Ok(Type::CNAME),
            15 => Ok(Type::MX),
            16 => Ok(Type::TXT),
            28 => Ok(Type::AAAA),
            _ => Err(DnsError::DecodeError(format!(
                "Integer not converted to a RR Type: {}",
                val
            ))),
        }
    }
}

// allow non-camel case to match the DNS name for these values
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[allow(non_camel_case_types)]
pub enum Class {
    CLASS_IN = 1,
}

impl TryFrom<u16> for Class {
    type Error = DnsError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        match val {
            1 => Ok(Class::CLASS_IN),
            _ => Err(DnsError::DecodeError(format!(
                "Integer not converted to a RR Class: {val}"
            ))),
        }
    }
}

#[allow(non_camel_case_types)]
enum HeaderFlags {
    QR_RESPONSE = 0b0000_0000_0000_0001, // Query on 0, Response on 1
    AA = 0b0000_0000_0001_0000,          // Authoritative Answer
    TC = 0b0000_0000_0010_0000,          // Truncation
    RD = 0b0000_0000_0100_0000,          // Recursion Desired
    RA = 0b0000_0000_1000_0000,          // Recursion Available
    RCODE_FORMAT_ERR = 0b0001_0000_0000_0000, // Failed to interpret format
    RCODE_SERVER_ERR = 0b0010_0000_0000_0000, // Server failure
    RCODE_NOT_IMPL = 0b0100_0000_0000_0000, // Not Implemented
    RCODE_REFUSED = 0b0101_0000_0000_0000, // Refused
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_try_from() {
        let vals = [1, 2, 5, 15, 16, 28];
        let converted = vals.map(|x| Type::try_from(x));
        let expected = [
            Ok(Type::A),
            Ok(Type::NS),
            Ok(Type::CNAME),
            Ok(Type::MX),
            Ok(Type::TXT),
            Ok(Type::AAAA),
        ];
        assert_eq!(converted, expected);
    }
    #[test]
    fn test_try_from_err() {
        let val = 65535;
        let converted = Type::try_from(val);
        assert!(converted.is_err());
    }
}
