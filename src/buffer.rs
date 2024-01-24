use std::fmt::{Debug, Display, Formatter};

pub struct LineRange {
    start: usize,
    end: usize,
}

impl Display for LineRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {})",
            self.start,
            self.end,
        )
    }
}

impl Debug for LineRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl LineRange {
    fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

pub struct Buffer {
    content: Vec<char>,
    lines: Vec<LineRange>,
    cursor: usize,
    line: usize,
}

impl Display for Buffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "buffer [ cursor: {}, line: {}, content: '{}', lines: '{:?}']",
            self.cursor,
            self.line,
            self.content.iter().collect::<String>(),
            self.lines,
        )
    }
}

impl Debug for Buffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "buffer [ cursor: {}, line: {}, lines: '{:?}']",
            self.cursor,
            self.line,
            self.lines,
        )
    }
}

impl Buffer {
    fn sync_line_with_cursor(&mut self, pos: usize) {
        self.cursor = pos;
        for (i, line) in self.lines.iter().enumerate() {
            if line.end >= self.cursor {
                self.line = i;
                break;
            }
        }
    }
    fn insert_updates(&mut self) {
        for line in self.line + 1..self.lines.len() {
            self.lines[line].start += 1;
            self.lines[line].end += 1;
        }
    }
    fn remove_updates(&mut self) {
        for line in self.line..self.lines.len() {
            let curr_line = &self.lines[line];
            if self.line != self.lines.len() - 1 && curr_line.start == curr_line.end && self.cursor == curr_line.start {
                continue
            }
            if line == self.line {
                self.lines[line].end -= 1;
                continue;
            }
            self.lines[line].start -= 1;
            self.lines[line].end -= 1;
        }
    }
    pub fn new() -> Self {
        Self {
            content: vec![],
            lines: vec![LineRange::new(0, 0)],
            cursor: 0,
            line: 0,
        }
    }
    pub fn from_str(content: &str) -> Self {
        let (mut last_lb, mut cursor, mut line, mut lines, content) = (0, 0, 0, vec![LineRange::new(0, 0)], content.chars().collect::<Vec<char>>());
        for (i, c) in content.iter().enumerate() {
            if '\n' == *c {
                lines[line] = LineRange::new(last_lb, i);
                line += 1;
                last_lb = i + 1;
                if lines.len() == line {
                    lines.push(LineRange::new(last_lb, last_lb));
                }
                cursor = i + 1;
                continue;
            }
            lines[line].end = i;
            cursor = i + 1;
        }
        let line = lines.len() - 1;
        Self { cursor, content, lines, line }
    }
    pub fn insert_at(&mut self, pos: usize, c: char) {
        self.sync_line_with_cursor(pos);
        match c {
            '\n' => {
                self.content.insert(self.cursor, c);
                self.lines[self.line].end = self.cursor;
                self.cursor += 1;
                self.line += 1;
                if self.lines.len() <= self.line {
                    self.lines.insert(self.line, LineRange::new(self.cursor, self.cursor));
                }
                self.insert_updates()
            }
            c => {
                self.content.insert(self.cursor, c);
                self.lines[self.line].end = self.cursor;
                self.cursor += 1;
            }
        }
    }
    pub fn insert(&mut self, c: char) {
        self.insert_at(self.cursor, c)
    }
    pub fn remove_at(&mut self, pos: usize) {
        if self.cursor == 0 { return; }
        self.sync_line_with_cursor(pos);
        match self.content[self.cursor] {
            '\n' => {
                self.content.remove(self.cursor);
                self.remove_updates();
                if self.line + 1 < self.lines.len() {
                    if self.lines[self.line].end < self.lines[self.line + 1].end {
                        self.lines[self.line].end = self.lines[self.line + 1].end;
                    }
                    if self.lines[self.line].end == self.cursor {
                        if self.lines[self.line].end != 0 {
                            self.lines[self.line].end -= 1;
                        }
                    }
                    self.lines.remove(self.line + 1);
                }
            }
            _ => {
                self.content.remove(self.cursor);
                if self.lines[self.line].end != 0 {
                    self.lines[self.line].end -= 1;
                }
            }
        }
    }
    pub fn remove(&mut self) {
        if self.cursor == 0 { return; }
        self.remove_at(self.cursor - 1)
    }
    pub fn content(&self) -> String {
        if self.content.len() == 0 {
            return String::new();
        }
        let mut content = String::new();
        for line in &self.lines {
            if line.start == self.content.len() {
                continue;
            }
            let mut line_content = self.content[line.start..=line.end].iter().collect::<String>();
            if line_content.ends_with('\n') {
                line_content.insert(line_content.len() - 1, '\r');
            }
            content.push_str(&line_content)
        }
        content
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_should_create_buffer_from_str() {
        let buffer = Buffer::from_str("package main");
        assert_eq!(
            "buffer [ cursor: 12, line: 0, content: 'package main', lines: '[(0, 11)]']",
            buffer.to_string(),
        );
    }

    #[test]
    fn test_should_create_from_str_with_multiple_lines() {
        let buffer = Buffer::from_str("package main\nfunc main() {\n}\n");
        assert_eq!(
            "buffer [ cursor: 29, line: 3, content: 'package main\nfunc main() {\n}\n', lines: '[(0, 12), (13, 26), (27, 28), (29, 29)]']",
            buffer.to_string(),
        );
    }

    #[test]
    fn test_should_insert_char_at_given_position() {
        let mut buffer = Buffer::new();
        assert_eq!(
            "buffer [ cursor: 0, line: 0, content: '', lines: '[(0, 0)]']",
            buffer.to_string()
        );
        buffer.insert_at(0, 'a');
        assert_eq!(
            "buffer [ cursor: 1, line: 0, content: 'a', lines: '[(0, 0)]']",
            buffer.to_string()
        );
    }

    #[test]
    fn test_should_insert_char_at_cursor() {
        let mut buffer = Buffer::new();
        assert_eq!(
            "buffer [ cursor: 0, line: 0, content: '', lines: '[(0, 0)]']",
            buffer.to_string()
        );
        buffer.insert('\n');
        assert_eq!(
            "buffer [ cursor: 1, line: 1, content: '\n', lines: '[(0, 0), (1, 1)]']",
            buffer.to_string()
        );
    }

    #[test]
    fn test_should_insert_new_line() {
        let mut buffer = Buffer::new();
        assert_eq!(
            "buffer [ cursor: 0, line: 0, content: '', lines: '[(0, 0)]']",
            buffer.to_string()
        );
        buffer.insert_at(0, '\n');
        assert_eq!(
            "buffer [ cursor: 1, line: 1, content: '\n', lines: '[(0, 0), (1, 1)]']",
            buffer.to_string()
        );
    }

    #[test]
    fn test_should_insert_new_line_at_given_position() {
        let mut buffer = Buffer::from_str("aa");
        assert_eq!(
            "buffer [ cursor: 2, line: 0, content: 'aa', lines: '[(0, 1)]']",
            buffer.to_string()
        );
        buffer.insert_at(1, '\n');
        assert_eq!(
            "buffer [ cursor: 2, line: 1, content: 'a\na', lines: '[(0, 1), (2, 2)]']",
            buffer.to_string()
        );
    }

    #[test]
    fn test_should_remove_char_at_given_position() {
        let mut buffer = Buffer::from_str("aa");
        assert_eq!(
            "buffer [ cursor: 2, line: 0, content: 'aa', lines: '[(0, 1)]']",
            buffer.to_string()
        );
        buffer.remove_at(0);
        assert_eq!(
            "buffer [ cursor: 0, line: 0, content: 'a', lines: '[(0, 0)]']",
            buffer.to_string()
        );
    }

    #[test]
    fn test_should_remove_char_before_cursor() {
        let mut buffer = Buffer::from_str("aa");
        assert_eq!(
            "buffer [ cursor: 2, line: 0, content: 'aa', lines: '[(0, 1)]']",
            buffer.to_string()
        );
        buffer.remove();
        assert_eq!(
            "buffer [ cursor: 1, line: 0, content: 'a', lines: '[(0, 0)]']",
            buffer.to_string()
        );
    }

    #[test]
    fn test_should_remove_new_line_at_given_position() {
        let mut buffer = Buffer::from_str("lorem\nipsum\ndolor");
        assert_eq!(
            "buffer [ cursor: 17, line: 2, content: 'lorem\nipsum\ndolor', lines: '[(0, 5), (6, 11), (12, 16)]']",
            buffer.to_string()
        );
        buffer.remove_at(5);
        assert_eq!(
            "buffer [ cursor: 5, line: 0, content: 'loremipsum\ndolor', lines: '[(0, 10), (11, 15)]']",
            buffer.to_string()
        );
    }

    #[test]
    fn test_should_remove_last_line() {
        let mut buffer = Buffer::from_str("lorem\nipsum\ndolor\n");
        assert_eq!(
            "buffer [ cursor: 18, line: 3, content: 'lorem\nipsum\ndolor\n', lines: '[(0, 5), (6, 11), (12, 17), (18, 18)]']",
            buffer.to_string()
        );
        buffer.remove_at(17);
        assert_eq!(
            "buffer [ cursor: 17, line: 2, content: 'lorem\nipsum\ndolor', lines: '[(0, 5), (6, 11), (12, 16)]']",
            buffer.to_string()
        );
    }

    #[test]
    fn test_should_correctly_select_line_for_pos() {
        let mut buffer = Buffer::from_str("lorem\nipsum");
        assert_eq!(
            "buffer [ cursor: 11, line: 1, content: 'lorem\nipsum', lines: '[(0, 5), (6, 10)]']",
            buffer.to_string()
        );
        buffer.sync_line_with_cursor(0);
        assert_eq!(
            "buffer [ cursor: 0, line: 0, content: 'lorem\nipsum', lines: '[(0, 5), (6, 10)]']",
            buffer.to_string()
        );
    }

    #[test]
    fn test_should_return_correct_content() {
        let mut buffer = Buffer::from_str("la vida de albert\n\n");
        assert_eq!("la vida de albert\r\n\r\n", buffer.content());
        buffer.remove();
        assert_eq!("la vida de albert\r\n", buffer.content());
        buffer.remove();
        assert_eq!("la vida de albert", buffer.content());
    }

    #[test]
    fn test_should_remove_only_new_line() {
        let mut buffer = Buffer::from_str("\n");
        assert_eq!("\r\n", buffer.content());
        buffer.remove();
        assert_eq!("", buffer.content());
    }
}