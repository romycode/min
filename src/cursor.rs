use std::fmt::{Debug, Display, Formatter};

pub struct Cursor {
    line: usize,
    column: usize,
    lines: usize,
    columns: usize,
}

impl Display for Cursor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "cursor[line:{},column:{}]", self.line, self.column)
    }
}

impl Debug for Cursor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Cursor {
    pub fn new(lines: usize, columns: usize) -> Self {
        Self { line: 0, column: 0, lines, columns }
    }
    pub fn left(&mut self) {
        if self.column == 0 {
            return;
        }
        self.column -= 1;
    }
    pub fn right(&mut self) {
        if self.column == self.columns {
            return;
        }
        self.column += 1;
    }
    pub fn up(&mut self) {
        if self.line == 0 {
            return;
        }
        self.line -= 1;
    }
    pub fn down(&mut self) {
        if self.line == self.lines {
            return;
        }
        self.line += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cursor_do_nothing_left_is_called_and_column_is_0() {
        let mut cursor = Cursor::new(10, 10);
        cursor.left();
        assert_eq!("cursor[line:0,column:0]", cursor.to_string());
    }

    #[test]
    fn test_cursor_do_nothing_right_is_called_and_column_is_limit() {
        let mut cursor = Cursor {
            line: 10,
            column: 10,
            lines: 10,
            columns: 10,
        };
        cursor.right();
        assert_eq!("cursor[line:10,column:10]", cursor.to_string());
    }

    #[test]
    fn test_cursor_move_left() {
        let mut cursor = Cursor {
            line: 10,
            column: 8,
            lines: 10,
            columns: 10,
        };
        cursor.left();
        assert_eq!("cursor[line:10,column:7]", cursor.to_string());
    }

    #[test]
    fn test_cursor_move_right() {
        let mut cursor = Cursor {
            line: 10,
            column: 9,
            lines: 10,
            columns: 10,
        };
        cursor.right();
        assert_eq!("cursor[line:10,column:10]", cursor.to_string());
    }

    #[test]
    fn test_cursor_do_nothing_up_is_called_and_line_is_limit() {
        let mut cursor = Cursor::new(10, 10);
        cursor.up();
        assert_eq!("cursor[line:0,column:0]", cursor.to_string());
    }

    #[test]
    fn test_cursor_do_nothing_down_is_called_and_line_is_0() {
        let mut cursor = Cursor {
            line: 10,
            column: 10,
            lines: 10,
            columns: 10,
        };
        cursor.right();
        assert_eq!("cursor[line:10,column:10]", cursor.to_string());
    }

    #[test]
    fn test_cursor_move_up() {
        let mut cursor = Cursor {
            line: 10,
            column: 10,
            lines: 10,
            columns: 10,
        };
        cursor.up();
        assert_eq!("cursor[line:9,column:10]", cursor.to_string());
    }

    #[test]
    fn test_cursor_move_down() {
        let mut cursor = Cursor {
            line: 9,
            column: 10,
            lines: 10,
            columns: 10,
        };
        cursor.down();
        assert_eq!("cursor[line:10,column:10]", cursor.to_string());
    }
}