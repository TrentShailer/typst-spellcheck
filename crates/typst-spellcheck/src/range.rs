use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.start.line == self.end.line {
            write!(
                f,
                "line {}, column {}-{}",
                self.start.line, self.start.column, self.end.column
            )
        } else {
            write!(
                f,
                "lines {}-{}, column {}-{}",
                self.start.line, self.end.line, self.start.column, self.end.column
            )
        }
    }
}
