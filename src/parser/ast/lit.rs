use super::{Span, Type};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Lit(pub LitKind);

impl Lit {
    pub fn new(lit_kind: LitKind) -> Self {
        Self(lit_kind)
    }

    pub fn is_true(&self) -> Option<bool> {
        match self.0 {
            LitKind::Bool(b) => Some(b),
            _ => None,
        }
    }
}

impl From<i32> for Lit {
    fn from(value: i32) -> Self {
        Self(LitKind::Num(value as f32))
    }
}

impl From<f32> for Lit {
    fn from(value: f32) -> Self {
        Self(LitKind::Num(value as f32))
    }
}

impl From<(i32, TimeKind)> for Lit {
    fn from(value: (i32, TimeKind)) -> Self {
        Self(LitKind::Time(value.0 as f32, value.1))
    }
}

impl From<(f32, TimeKind)> for Lit {
    fn from(value: (f32, TimeKind)) -> Self {
        Self(LitKind::Time(value.0, value.1))
    }
}

impl From<bool> for Lit {
    fn from(value: bool) -> Self {
        Self(LitKind::Bool(value))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LitKind {
    /// Number (Float or Int)
    Num(f32),
    /// Time
    Time(f32, TimeKind),
    // Boolean
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TimeKind {
    /// Milliseconds
    Ms,
    /// Seconds
    Sec,
    /// Minute
    Min,
    /// Hour
    Hour,
    /// Day
    Day,
    /// Week
    Week,
    /// Year
    Year,
}

impl TimeKind {
    pub fn as_ms(&self) -> f32 {
        match self {
            TimeKind::Ms => 1.0,
            TimeKind::Sec => 1000.0,
            TimeKind::Min => 3600.0,
            TimeKind::Hour => 3_600_000.0,
            TimeKind::Day => 86_400_000.0,
            TimeKind::Week => 604_800_000.0,
            TimeKind::Year => 31_536_000_000.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_lit_works() {
        let num_lit = Lit::new(LitKind::Num(3.));
        let time_lit = Lit::new(LitKind::Time(3., TimeKind::Sec));
        let bool_lit = Lit::new(LitKind::Bool(true));

        assert_eq!(num_lit, Lit::from(3));
        assert_eq!(num_lit, Lit::from(3.0));
        assert_eq!(time_lit, Lit::from((3, TimeKind::Sec)));
        assert_eq!(time_lit, Lit::from((3.0, TimeKind::Sec)));
        assert_eq!(bool_lit, Lit::from(true));
    }
}
