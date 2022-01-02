use super::*;

impl From<LengthUnit> for UnitValue {
    fn from(n: LengthUnit) -> Self {
        Self::Length(n)
    }
}

impl From<&str> for UnitValue {
    fn from(s: &str) -> Self {
        Self::Keyword(s.to_string())
    }
}

impl From<String> for UnitValue {
    fn from(s: String) -> Self {
        Self::Keyword(s)
    }
}

impl From<TailwindArbitrary> for UnitValue {
    fn from(a: TailwindArbitrary) -> Self {
        Self::Arbitrary(a)
    }
}

impl From<&TailwindArbitrary> for UnitValue {
    fn from(a: &TailwindArbitrary) -> Self {
        Self::Arbitrary(a.clone())
    }
}
