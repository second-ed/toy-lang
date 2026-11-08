#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Keyword {
    Fn,
    If,
    Else,
    Return,
    True,
    False,
    For,
    In,
}

impl Keyword {
    pub(crate) fn from_str(text: &str) -> Option<Self> {
        match text {
            "fn" => Some(Self::Fn),
            "if" => Some(Self::If),
            "else" => Some(Self::Else),
            "return" => Some(Self::Return),
            "true" => Some(Self::True),
            "false" => Some(Self::False),
            "for" => Some(Self::For),
            "in" => Some(Self::In),
            _ => None,
        }
    }
}
