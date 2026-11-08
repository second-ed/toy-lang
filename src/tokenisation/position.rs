#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Position {
    line: u64,
    col: u64,
}

impl Position {
    pub(crate) fn new(line: u64, col: u64) -> Self {
        Self { line, col }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Span {
    pub(crate) start_pos: Position,
    pub(crate) end_pos: Position,
}

impl Span {
    pub(crate) fn new(start_pos: Position, end_pos: Position) -> Self {
        Self { start_pos, end_pos }
    }
}
