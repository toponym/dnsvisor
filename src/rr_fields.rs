/// Enums with values for DNS Resource Record (RR) fields

pub enum Type {
    A = 1,
}
// allow non-camel case to match the DNS name for these values
#[allow(non_camel_case_types)]
pub enum Class {
    CLASS_IN = 1,
}
