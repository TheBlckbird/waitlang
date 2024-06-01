use super::{Span, Type};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Lit {
    pub lit_kind: LitKind,
    pub span: Span,
    pub type_: Type,
}

impl Lit {
    pub fn new(lit_kind: LitKind, span: Span, type_: Type) -> Self {
        Self {
            lit_kind,
            span,
            type_,
        }
    }

    pub fn is_true(&self) -> Option<bool> {
        match self.lit_kind {
            LitKind::Bool(b) => Some(b),
            _ => None,
        }
    }
}

impl From<(i32, Span, Type)> for Lit {
    fn from(value: (i32, Span, Type)) -> Self {
        Self {
            lit_kind: LitKind::Num(value.0 as f32),
            span: value.1,
            type_: value.2,
        }
    }
}

impl From<(f32, Span, Type)> for Lit {
    fn from(value: (f32, Span, Type)) -> Self {
        Self {
            lit_kind: LitKind::Num(value.0),
            span: value.1,
            type_: value.2,
        }
    }
}

impl From<(i32, TimeKind, Span, Type)> for Lit {
    fn from(value: (i32, TimeKind, Span, Type)) -> Self {
        Self {
            lit_kind: LitKind::Time(value.0 as f32, value.1),
            span: value.2,
            type_: value.3,
        }
    }
}

impl From<(f32, TimeKind, Span, Type)> for Lit {
    fn from(value: (f32, TimeKind, Span, Type)) -> Self {
        Self {
            lit_kind: LitKind::Time(value.0, value.1),
            span: value.2,
            type_: value.3,
        }
    }
}

impl From<(bool, Span, Type)> for Lit {
    fn from(value: (bool, Span, Type)) -> Self {
        Self {
            lit_kind: LitKind::Bool(value.0),
            span: value.1,
            type_: value.2,
        }
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
            TimeKind::Ms => 1.,
            TimeKind::Sec => 1000.,
            TimeKind::Min => 3600.,
            TimeKind::Hour => 3_600_000.,
            TimeKind::Day => 86_400_000.,
            TimeKind::Week => 604_800_000.,
            TimeKind::Year => 31_536_000_000.,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_lit_works() {
        let num_lit = Lit::new(LitKind::Num(3.), Span::from((1, 1)), Type::Number);
        let time_lit = Lit::new(
            LitKind::Time(3., TimeKind::Sec),
            Span::from((1, 1)),
            Type::Time,
        );
        let bool_lit = Lit::new(LitKind::Bool(true), Span::from((1, 1)), Type::Bool);

        assert_eq!(num_lit, Lit::from((3, Span::from((1, 1)), Type::Number)));
        assert_eq!(num_lit, Lit::from((3., Span::from((1, 1)), Type::Number)));
        assert_eq!(
            time_lit,
            Lit::from((3, TimeKind::Sec, Span::from((1, 1)), Type::Time))
        );
        assert_eq!(
            time_lit,
            Lit::from((3., TimeKind::Sec, Span::from((1, 1)), Type::Time))
        );
        assert_eq!(bool_lit, Lit::from((true, Span::from((1, 1)), Type::Bool)));
    }
}
