use super::*;

pub(crate) mod bottom;
pub(crate) mod inset;
pub(crate) mod left;
pub(crate) mod right;
pub(crate) mod top;

#[derive(Debug, Clone)]
enum PlacementSize {
    Auto,
    Full,
    Length(LengthUnit),
    Standard(StandardValue),
}

impl Display for PlacementSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Full => write!(f, "full"),
            Self::Length(n) if n.is_fraction() => write!(f, "{}", n.get_class()),
            Self::Length(n) => write!(f, "{}", n.get_class_arbitrary()),
            Self::Standard(s) => write!(f, "{}", s),
        }
    }
}

impl PlacementSize {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            ["px"] => Self::Length(LengthUnit::px(1.0)),
            ["full"] => Self::Full,
            ["auto"] => Self::Auto,
            [n] => {
                let a = TailwindArbitrary::from(*n);
                Self::maybe_no_unit(&a).or_else(|_| Self::maybe_length(&a))?
            },
            _ => return syntax_error!("Unknown placement instructions: {}", pattern.join("-")),
        };
        Ok(kind)
    }
    pub fn get_properties(&self) -> String {
        match self {
            Self::Auto => "auto".to_string(),
            Self::Full => "full".to_string(),
            Self::Length(x) => x.get_properties(),
            Self::Standard(x) => x.get_properties().to_string(),
        }
    }
    fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Standard(StandardValue::parse_arbitrary(arbitrary)?))
    }
    #[inline]
    fn maybe_length(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Length(arbitrary.as_length()?))
    }
    #[inline]
    fn maybe_no_unit(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Length(LengthUnit::rem(arbitrary.as_float()? / 4.0)))
    }
}
