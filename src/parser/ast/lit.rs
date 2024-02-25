use super::Span;

#[derive(Debug, PartialEq)]
pub struct Lit {
    pub lit_kind: LitKind,
    pub span: Span,
}

impl Lit {
    pub fn new(lit_kind: LitKind, span: Span) -> Self {
        Self { lit_kind, span }
    }
}

impl From<(i32, Span)> for Lit {
    fn from(value: (i32, Span)) -> Self {
        Self {
            lit_kind: LitKind::Num(value.0 as f32),
            span: value.1,
        }
    }
}

impl From<(f32, Span)> for Lit {
    fn from(value: (f32, Span)) -> Self {
        Self {
            lit_kind: LitKind::Num(value.0),
            span: value.1,
        }
    }
}

impl From<(i32, TimeKind, Span)> for Lit {
    fn from(value: (i32, TimeKind, Span)) -> Self {
        Self {
            lit_kind: LitKind::Time(value.0 as f32, value.1),
            span: value.2,
        }
    }
}

impl From<(f32, TimeKind, Span)> for Lit {
    fn from(value: (f32, TimeKind, Span)) -> Self {
        Self {
            lit_kind: LitKind::Time(value.0, value.1),
            span: value.2,
        }
    }
}

impl From<(bool, Span)> for Lit {
    fn from(value: (bool, Span)) -> Self {
        Self {
            lit_kind: LitKind::Bool(value.0),
            span: value.1,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LitKind {
    /// Number (Float or Int)
    Num(f32),
    /// Time
    Time(f32, TimeKind),
    // Boolean
    Bool(bool),
}

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_lit_works() {
        let num_lit = Lit::new(LitKind::Num(3.), Span::from((1, 1)));
        let time_lit = Lit::new(LitKind::Time(3., TimeKind::Sec), Span::from((1, 1)));
        let bool_lit = Lit::new(LitKind::Bool(true), Span::from((1, 1)));

        assert_eq!(num_lit, Lit::from((3, Span::from((1, 1)))));
        assert_eq!(num_lit, Lit::from((3., Span::from((1, 1)))));
        assert_eq!(time_lit, Lit::from((3, TimeKind::Sec, Span::from((1, 1)))));
        assert_eq!(time_lit, Lit::from((3., TimeKind::Sec, Span::from((1, 1)))));
        assert_eq!(bool_lit, Lit::from((true, Span::from((1, 1)))));
    }
}