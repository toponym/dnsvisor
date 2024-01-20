use crate::error::DnsError;
/// Enums with values for DNS Resource Record (RR) fields
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Type {
    A = 1,
    NS = 2,
    CNAME = 5,
    SOA = 6,
    WKS = 11,
    PTR = 12,
    HINFO = 13,
    MINFO = 14,
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
            6 => Ok(Type::SOA),
            11 => Ok(Type::WKS),
            12 => Ok(Type::PTR),
            13 => Ok(Type::HINFO),
            14 => Ok(Type::MINFO),
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

// TODO use a bitfield/bitmask library
#[allow(non_camel_case_types)]
pub enum HeaderFlags {
    QR_RESPONSE = 0x8000,      // Query on 0, Response on 1
    RCODE_FORMAT_ERR = 0x0001, // Failed to interpret format
    RCODE_SERVER_ERR = 0x0002, // Server failure
    RCODE_NOT_IMPL = 0x0004,   // Not Implemented
    RCODE_REFUSED = 0x0005,    // Refused
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
