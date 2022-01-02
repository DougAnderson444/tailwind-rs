use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBlur {
    px: NumericValue,
    backdrop: Backdrop,
}
impl Display for TailwindBlur {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.backdrop.write(f)?;
        write!(f, "blur-{}", self.px)
    }
}

impl TailwindInstance for TailwindBlur {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let class = self.backdrop.filter();
        let value = match &self.px {
            NumericValue::Number(n, _) => format!("blur({}px)", n),
            NumericValue::Arbitrary(n) => format!("blur({})", n.get_properties()),
            NumericValue::Keyword(_) => unreachable!(),
        };
        css_attributes! {
            class => value
        }
    }
}

impl TailwindBlur {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let px = match rest {
            ["none"] => 0usize.into(),
            ["sm"] => 4usize.into(),
            ["base"] => 8usize.into(),
            [] => {
                debug_assert!(arbitrary.is_none());
                8usize.into()
            },
            ["md"] => 12usize.into(),
            ["lg"] => 16usize.into(),
            ["xl"] => 24usize.into(),
            ["2xl"] => 40usize.into(),
            ["3xl"] => 64usize.into(),
            _ => NumericValue::positive_parser("blur")(rest, arbitrary)?,
        };
        Ok(Self { px, backdrop: Backdrop::from(backdrop) })
    }
}
